import chess
import chess.pgn
import numpy as np
import torch
import io

def board_to_representation(board: chess.Board) -> np.ndarray:
    """
    Converts a chess.Board object into a 19x8x8 numpy array representation.
    
    The 19 planes are:
    - 0-5: White pieces (P, N, B, R, Q, K)
    - 6-11: Black pieces (P, N, B, R, Q, K)
    - 12: Player to move (1 for white, 0 for black)
    - 13: White kingside castling rights
    - 14: White queenside castling rights
    - 15: Black kingside castling rights
    - 16: Black queenside castling rights
    - 17: En passant square
    - 18: Fifty-move rule counter
    """
    
    # Initialize a 19x8x8 array of zeros.
    # Using float32 for compatibility with PyTorch tensors.
    representation = np.zeros((19, 8, 8), dtype=np.float32)
    
    # --- 0-11: Piece Positions ---
    for piece_type in chess.PIECE_TYPES: # 1-6 (Pawn to King)
        for color in chess.COLORS: # True for White, False for Black
            # Get a bitmask of pieces of a certain type and color
            bitboard = board.pieces_mask(piece_type, color)
            
            # Calculate the plane index
            # White pieces are planes 0-5, Black are 6-11
            plane_idx = (piece_type - 1) + (0 if color == chess.WHITE else 6)

            # Populate the plane
            for square in range(64):
                if (bitboard >> square) & 1:
                    row = square // 8
                    col = square % 8
                    representation[plane_idx, row, col] = 1

    # --- 12: Player to move ---
    # 1.0 for White's turn, 0.0 for Black's turn
    if board.turn == chess.WHITE:
        representation[12, :, :] = 1.0
    else:
        representation[12, :, :] = 0.0

    # --- 13-16: Castling Rights ---
    if board.has_kingside_castling_rights(chess.WHITE):
        representation[13, :, :] = 1.0
    if board.has_queenside_castling_rights(chess.WHITE):
        representation[14, :, :] = 1.0
    if board.has_kingside_castling_rights(chess.BLACK):
        representation[15, :, :] = 1.0
    if board.has_queenside_castling_rights(chess.BLACK):
        representation[16, :, :] = 1.0
        
    # --- 17: En Passant Square ---
    if board.ep_square is not None:
        row = board.ep_square // 8
        col = board.ep_square % 8
        representation[17, row, col] = 1.0

    # --- 18: Fifty-move Rule Counter ---
    # Normalize the counter to be between 0 and 1
    representation[18, :, :] = board.halfmove_clock / 100.0
    
    return representation


# --- Move Representation Constants (Based on AlphaZero) ---

# 8 directions for queen moves
DIRECTIONS = {
    'N': (1, 0), 'NE': (1, 1), 'E': (0, 1), 'SE': (-1, 1),
    'S': (-1, 0), 'SW': (-1, -1), 'W': (0, -1), 'NW': (1, -1)
}
DIRECTION_TO_IDX = {name: i for i, name in enumerate(DIRECTIONS)}
IDX_TO_DIRECTION = {i: name for i, name in enumerate(DIRECTIONS)}

# 8 moves for a knight
KNIGHT_MOVES = [
    (2, 1), (1, 2), (-1, 2), (-2, 1),
    (-2, -1), (-1, -2), (1, -2), (2, -1)
]
KNIGHT_MOVE_TO_IDX = {move: i for i, move in enumerate(KNIGHT_MOVES)}

# Total planes/channels for move types
# 56 queen-like + 8 knight + 9 underpromotion = 73
NUM_MOVE_PLANES = 73

# Create a mapping from UCI string to index (0-4671)
MOVE_TO_INDEX = {}
INDEX_TO_MOVE = {}

def _initialize_move_maps():
    """
    Populates the MOVE_TO_INDEX and INDEX_TO_MOVE dictionaries.
    This function creates the definitive mapping between a move and its unique index.
    """
    if MOVE_TO_INDEX and INDEX_TO_MOVE: # Avoid re-computation
        return
        
    plane_idx = 0
    # 1. Queen-like moves (56 planes)
    for direction_idx in range(8):
        for distance in range(1, 8): # 1 to 7 squares
            for from_sq in range(64):
                from_rank, from_file = divmod(from_sq, 8)
                direction = IDX_TO_DIRECTION[direction_idx]
                dr, df = DIRECTIONS[direction]
                
                to_rank, to_file = from_rank + dr * distance, from_file + df * distance
                
                if 0 <= to_rank < 8 and 0 <= to_file < 8:
                    to_sq = to_rank * 8 + to_file
                    move = chess.Move(from_sq, to_sq)
                    index = from_sq * NUM_MOVE_PLANES + plane_idx
                    MOVE_TO_INDEX[move.uci()] = index
                    INDEX_TO_MOVE[index] = move.uci()
            plane_idx += 1

    # 2. Knight moves (8 planes)
    for knight_move_idx in range(8):
        for from_sq in range(64):
            from_rank, from_file = divmod(from_sq, 8)
            dr, df = KNIGHT_MOVES[knight_move_idx]
            
            to_rank, to_file = from_rank + dr, from_file + df
            
            if 0 <= to_rank < 8 and 0 <= to_file < 8:
                to_sq = to_rank * 8 + to_file
                move = chess.Move(from_sq, to_sq)
                index = from_sq * NUM_MOVE_PLANES + plane_idx
                MOVE_TO_INDEX[move.uci()] = index
                INDEX_TO_MOVE[index] = move.uci()
        plane_idx += 1
        
    # 3. Underpromotions (9 planes)
    # Promotions to Queen are handled by queen moves
    promotion_pieces = [chess.KNIGHT, chess.BISHOP, chess.ROOK]
    promotion_deltas = [-1, 0, 1] # capture left, forward, capture right
    
    for promotion_piece in promotion_pieces:
        for delta_file in promotion_deltas:
            for from_file in range(8):
                # White promotions
                from_sq_w = 6 * 8 + from_file
                to_file_w = from_file + delta_file
                if 0 <= to_file_w < 8:
                    to_sq_w = 7 * 8 + to_file_w
                    move_w = chess.Move(from_sq_w, to_sq_w, promotion=promotion_piece)
                    index_w = from_sq_w * NUM_MOVE_PLANES + plane_idx
                    MOVE_TO_INDEX[move_w.uci()] = index_w
                    INDEX_TO_MOVE[index_w] = move_w.uci()

                # Black promotions
                from_sq_b = 1 * 8 + from_file
                to_file_b = from_file + delta_file
                if 0 <= to_file_b < 8:
                    to_sq_b = 0 * 8 + to_file_b
                    move_b = chess.Move(from_sq_b, to_sq_b, promotion=promotion_piece)
                    index_b = from_sq_b * NUM_MOVE_PLANES + plane_idx
                    MOVE_TO_INDEX[move_b.uci()] = index_b
                    INDEX_TO_MOVE[index_b] = move_b.uci()

            plane_idx += 1

def move_to_index(move: chess.Move) -> int:
    """
    Converts a chess.Move object to its corresponding index (0-4671).
    """
    # Ensure the maps are populated
    _initialize_move_maps()
    
    # Handle regular promotions to queen, which are not special in our mapping
    uci_move = move.uci()
    if move.promotion == chess.QUEEN:
        uci_move = uci_move[:-1] # remove the 'q'
    
    return MOVE_TO_INDEX.get(uci_move)
