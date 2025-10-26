import torch
import torch.nn as nn
import torch.nn.functional as F

class ResidualBlock(nn.Module):
    """
    A standard residual block for a ResNet architecture.
    It consists of two convolutional layers with batch normalization and ReLU activation.
    The input is added to the output of the second convolutional layer (skip connection).
    """
    def __init__(self, in_channels, out_channels, stride=1):
        super(ResidualBlock, self).__init__()
        # First convolutional layer
        self.conv1 = nn.Conv2d(in_channels, out_channels, kernel_size=3, stride=stride, padding=1, bias=False)
        self.bn1 = nn.BatchNorm2d(out_channels)
        
        # Second convolutional layer
        self.conv2 = nn.Conv2d(out_channels, out_channels, kernel_size=3, stride=1, padding=1, bias=False)
        self.bn2 = nn.BatchNorm2d(out_channels)

        # Shortcut connection to match dimensions if necessary
        self.shortcut = nn.Sequential()
        if stride != 1 or in_channels != out_channels:
            self.shortcut = nn.Sequential(
                nn.Conv2d(in_channels, out_channels, kernel_size=1, stride=stride, bias=False),
                nn.BatchNorm2d(out_channels)
            )

    def forward(self, x):
        # Main path
        out = F.relu(self.bn1(self.conv1(x)))
        out = self.bn2(self.conv2(out))
        
        # Add the shortcut (skip connection)
        out += self.shortcut(x)
        out = F.relu(out)
        return out

class ChessResNet(nn.Module):
    """
    A ResNet architecture tailored for chess, featuring a value and a policy head.
    This model takes a stack of bitboards as input.
    """
    def __init__(self, num_input_channels=19, num_residual_blocks=19, num_filters=256, num_policy_moves=4672):
        """
        Initializes the Chess ResNet model.
        Args:
            num_input_channels (int): Number of planes in the input tensor (e.g., piece positions, castling rights).
            num_residual_blocks (int): Number of residual blocks in the network body.
            num_filters (int): Number of filters used in the convolutional layers.
            num_policy_moves (int): The number of possible moves to be predicted by the policy head.
                                    A common representation for chess is 4672.
        """
        super(ChessResNet, self).__init__()
        
        # 1. Initial Convolutional Block
        # This block processes the input bitboards.
        self.initial_conv = nn.Sequential(
            nn.Conv2d(num_input_channels, num_filters, kernel_size=3, stride=1, padding=1, bias=False),
            nn.BatchNorm2d(num_filters),
            nn.ReLU()
        )
        
        # 2. Body of Residual Blocks
        # This is the main part of the network that learns deep features.
        self.residual_tower = nn.Sequential(
            *[ResidualBlock(num_filters, num_filters) for _ in range(num_residual_blocks)]
        )
        
        # 3. Policy Head
        # This head predicts the probability distribution over all possible moves.
        
        self.policy_head = nn.Sequential(
            nn.Conv2d(num_filters, 2, kernel_size=1, stride=1, bias=False),
            nn.BatchNorm2d(2),
            nn.ReLU(),
            nn.Flatten(),
            nn.Linear(2 * 8 * 8, num_policy_moves)
            # The output will be passed through a log_softmax in the forward pass
        )
        
        # 4. Value Head
        # This head evaluates the current board position, outputting a single value
        # between -1 (black wins) and 1 (white wins).
        self.value_head = nn.Sequential(
            nn.Conv2d(num_filters, 1, kernel_size=1), # value head
            nn.BatchNorm2d(1),
            nn.Flatten(),
            nn.Linear(8*8, 64),
            nn.Linear(64, 1),
        )
        """
        self.value_head = nn.Sequential(
            nn.Conv2d(num_filters, 1, kernel_size=1, stride=1, bias=False),
            nn.BatchNorm2d(1),
            nn.ReLU(),
            nn.Flatten(),
            nn.Linear(1 * 8 * 8, 256),
            nn.ReLU(),
            nn.Linear(256, 1),
            nn.Sigmoid()
        )
        """

    def forward(self, x):
        """
        Defines the forward pass of the network.
        Args:
            x (torch.Tensor): The input tensor representing the board state.
                              Shape: (batch_size, num_input_channels, 8, 8)
        Returns:
            tuple: A tuple containing:
                - policy_logits (torch.Tensor): Raw output for the policy head.
                - value (torch.Tensor): The evaluation of the board position.
        """
        # Pass input through the initial convolutional block
        out = self.initial_conv(x)
        
        # Pass through the tower of residual blocks
        out = self.residual_tower(out)
        
        # Calculate policy and value
        policy_logits = self.policy_head(out)
        value = self.value_head(out)
        
        # The log_softmax for the policy is often applied outside the model
        # in the loss function (e.g., using nn.CrossEntropyLoss which combines log_softmax and nll_loss)
        return policy_logits, value

def main():
    """
    Example of how to instantiate and use the ChessResNet model.
    """
    # --- Model Configuration ---
    # Number of bitboard planes for input
    # (6 white pieces + 6 black pieces + 1 turn + 4 castling + 1 en passant + 1 fifty-move rule)
    input_channels = 19
    
    # Number of possible moves in a chess game used by AlphaZero.
    # This can be adjusted based on the specific move representation.
    policy_moves = 4672
    
    # Create an instance of the model
    model = ChessResNet(num_input_channels=input_channels, num_policy_moves=policy_moves)
    
    # --- Dummy Input ---
    # Create a random tensor to simulate a batch of 4 board states.
    # The shape is (batch_size, channels, height, width).
    batch_size = 4
    dummy_input = torch.randn(batch_size, input_channels, 8, 8)
    
    # --- Forward Pass ---
    # Set the model to evaluation mode
    model.eval()
    
    # Perform a forward pass without calculating gradients
    with torch.no_grad():
        policy_logits, value = model(dummy_input)
    
    # --- Print Output Shapes ---
    print("--- ChessResNet Architecture ---")
    print(f"Number of parameters: {sum(p.numel() for p in model.parameters() if p.requires_grad):,}")
    print("\n--- Input and Output Shapes ---")
    print(f"Input shape:  {dummy_input.shape}")
    print(f"Policy logits shape: {policy_logits.shape}")
    print(f"Value shape:  {value.shape}")

    # --- Print Example Output ---
    print("\n--- Example Output for a Single State ---")
    # Apply softmax to logits to get probabilities for the first state in the batch
    policy_probabilities = F.softmax(policy_logits[0], dim=0)
    
    print(f"Predicted value for the first state: {value[0].item():.4f}")
    print(f"Predicted policy probabilities sum: {policy_probabilities.sum().item():.4f}")
    # Find the move with the highest predicted probability
    best_move_index = torch.argmax(policy_probabilities).item()
    print(f"Highest probability move index: {best_move_index} with probability {policy_probabilities[best_move_index]:.4f}")

if __name__ == '__main__':
    main()
