from torch.utils.data import Dataset, DataLoader
import glob
import os
from tqdm import tqdm
import torch
import random


class ChessDataset(Dataset):
    """
    Custom PyTorch Dataset for loading pre-processed chess data.
    """
    def __init__(self, data_directory, samples_per_epoch=1000000):
        self.data_directory = data_directory
        self.samples_per_epoch = samples_per_epoch
        # Find all data chunk files
        self.file_list = glob.glob(os.path.join(data_directory, "data_chunk_*.pt"))
        random.shuffle(self.file_list)
        
        print(f"Found {len(self.file_list)} data files. Loading into memory...")
        
        self.data = []
        for f in tqdm(self.file_list, desc="Loading data files"):
            # Load each chunk and add its contents (a list of dicts) to our main data list
            self.data.extend(torch.load(f, weights_only=False))
            if len(self.data) >= self.samples_per_epoch:
                break
            
        print(f"Total positions loaded: {len(self.data)}")

    def __len__(self):
        return len(self.data)

    def __getitem__(self, idx):
        sample = self.data[idx]
        
        # Extract data from the dictionary
        representation = sample['representation']
        move_index = sample['policy']
        result = sample['value']
        
        # Convert to PyTorch tensors
        # The representation is already a numpy array, perfect for
        # torch.tensor(..., dtype=torch.float32)
        state_tensor = torch.tensor(representation, dtype=torch.float32)
        
        # The move index is our policy target
        # Needs to be a Long tensor for CrossEntropyLoss
        policy_tensor = torch.tensor(move_index, dtype=torch.long)
        
        # The result is our value target
        # Needs to be a Float tensor for MSELoss
        value_tensor = torch.tensor(result, dtype=torch.float32)
        
        return state_tensor, policy_tensor, value_tensor

    def resample(self):
        random.shuffle(self.file_list)
        
        print(f"Found {len(self.file_list)} data files. Loading into memory...")
        
        self.data = []
        for f in tqdm(self.file_list, desc="Loading data files"):
            # Load each chunk and add its contents (a list of dicts) to our main data list
            self.data.extend(torch.load(f, weights_only=False))
            if len(self.data) >= self.samples_per_epoch:
                break