def base_to_bits(base: str) -> int:
    mapping = {
        'A': 0b00,
        'C': 0b01,
        'G': 0b10,
        'T': 0b11
    }
    if base not in mapping:
        raise ValueError(f"Invalid base: {base}")
    return mapping[base]

def sequence_to_u64(seq: str) -> int:
    result = 0
    for base in seq:
        result <<= 2  # Shift left by 2 bits to make room
        result |= base_to_bits(base)
    return result


encoded = []
sequence = "ATGGCACCATGTCTGGTGAGCGCATCTACGGGAGCAATGGGTTCCCTCCTGACAAAGTTGGAAACCATGC"

k = 31
sequences = [sequence[i:i+k] for i in range(len(sequence) - k + 1)]

for sequence in sequences:
    encoded.append(str(sequence_to_u64(sequence)))

with open("kmers.txt", "w") as file:
    file.write("\n".join(encoded))
