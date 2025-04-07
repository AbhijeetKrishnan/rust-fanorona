def visualize_bits(number):
    """
    Visualizes the first 45 bits of an integer as a 5x9 grid printed to console.
    The bottom row contains bits 0-8, the next row contains bits 9-17, and so on.

    Args:
        number: The integer to visualize
    """
    # Create a 5x9 grid
    grid = [[0 for _ in range(9)] for _ in range(5)]

    # Fill the grid with bits
    for i in range(45):
        # Calculate row and column
        # Row: 4 (bottom) for bits 0-8, 3 for bits 9-17, etc.
        row = 4 - (i // 9)
        # Column: 0 for bits 0, 9, 18, etc.; 1 for bits 1, 10, 19, etc.
        col = i % 9

        # Extract the bit value
        bit_value = (number >> i) & 1
        grid[row][col] = bit_value

    # Print bit positions for reference
    print("Bit positions:")
    for i in range(5):
        row_start = (4 - i) * 9
        positions = " ".join(f"{row_start + j:2d}" for j in range(9))
        print(f"Row {i}: [{positions}]")

    print("\nBit grid visualization:")
    # Print the grid
    for i, row in enumerate(grid):
        row_str = " ".join(str(bit) for bit in row)
        print(f"Row {i}: [{row_str}]")


# Example usage
if __name__ == "__main__":
    # Example with a specific number
    example_number = 35184281124864
    print(f"Visualizing bits for: {example_number}")
    print(f"Binary representation: {bin(example_number)[2:].zfill(45)}")
    visualize_bits(example_number)

    # You can also get user input
    try:
        user_input = input(
            "\nEnter an integer to visualize (or press Enter to exit): "
        )
        if user_input:
            user_number = int(user_input)
            visualize_bits(user_number)
    except ValueError:
        print("Please enter a valid integer.")
