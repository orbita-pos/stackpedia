"use client";

// Simple hash from string to number
function hashCode(str: string): number {
  let hash = 0;
  for (let i = 0; i < str.length; i++) {
    const char = str.charCodeAt(i);
    hash = ((hash << 5) - hash + char) | 0;
  }
  return Math.abs(hash);
}

// Generate a symmetric 5x5 grid from a nickname
function generateGrid(nickname: string): boolean[][] {
  const hash = hashCode(nickname.toLowerCase());
  const grid: boolean[][] = [];

  for (let row = 0; row < 5; row++) {
    grid[row] = [];
    for (let col = 0; col < 3; col++) {
      // Use different bits of the hash for each cell
      const bit = (row * 3 + col);
      const seed = hashCode(nickname + bit.toString());
      grid[row][col] = seed % 3 !== 0; // ~66% fill rate
    }
    // Mirror: col 3 = col 1, col 4 = col 0
    grid[row][3] = grid[row][1];
    grid[row][4] = grid[row][0];
  }

  return grid;
}

// Pick a color based on nickname hash
const COLORS = [
  "#10b981", // emerald
  "#3b82f6", // blue
  "#8b5cf6", // violet
  "#f59e0b", // amber
  "#ef4444", // red
  "#ec4899", // pink
  "#06b6d4", // cyan
  "#84cc16", // lime
];

function getColor(nickname: string): string {
  const hash = hashCode(nickname);
  return COLORS[hash % COLORS.length];
}

interface PixelAvatarProps {
  nickname: string;
  size?: number;
}

export function PixelAvatar({ nickname, size = 24 }: PixelAvatarProps) {
  const grid = generateGrid(nickname);
  const color = getColor(nickname);
  const cellSize = size / 7; // 5 cells + 1px padding each side
  const offset = cellSize; // padding

  return (
    <div
      className="flex-shrink-0 bg-neutral-200 dark:bg-neutral-800 rounded-sm"
      style={{ width: size, height: size }}
    >
      <svg
        width={size}
        height={size}
        viewBox={`0 0 ${size} ${size}`}
        style={{ imageRendering: "pixelated" }}
      >
        {grid.map((row, y) =>
          row.map((filled, x) =>
            filled ? (
              <rect
                key={`${x}-${y}`}
                x={offset + x * cellSize}
                y={offset + y * cellSize}
                width={cellSize}
                height={cellSize}
                fill={color}
              />
            ) : null
          )
        )}
      </svg>
    </div>
  );
}
