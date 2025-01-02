using System.ComponentModel.DataAnnotations;
using System.Text.RegularExpressions;

namespace CSharp.Day4
{
    internal class Solution(string file) : BaseSolution(file)
    {
        public override int Run()
        {
            IEnumerable<string> lines = ReadFrom(file);

            List<List<char>> matrix = MakeMatrix(lines);

            return PartTwo(matrix);
        }

        private static List<List<char>> MakeMatrix(IEnumerable<string> lines)
        {
            List<List<char>> matrix = [];

            foreach (string line in lines)
            {
                matrix.Add([.. line.ToCharArray()]);
            }

            return matrix;
        }

        private static int PartTwo(List<List<char>> matrix)
        {
            int m = matrix.Count;
            int n = matrix[0].Count;

            int count = 0;

            for (int i = 0; i < m; i++)
            {
                for (int j = 0; j < n; j++)
                {
                    if (matrix[i][j] == 'M' || matrix[i][j] == 'S')
                    {
                        if (IsXMas(matrix, i, j))
                        {
                            count++;
                        }
                    }
                }
            }

            return count;
        }

        private static bool IsXMas(List<List<char>> matrix, int i, int j)
        {
            int m = matrix.Count;
            int n = matrix[0].Count;

            if (i >= m - 2 || j >= n - 2 || i < 0 || j < 0 || (matrix[i][j] != 'M' && matrix[i][j] != 'S'))
            {
                return false;
            }

            string upToBottom = $"{matrix[i][j]}{matrix[i + 1][j + 1]}{matrix[i + 2][j + 2]}";
            string bottomToUp = $"{matrix[i + 2][j]}{matrix[i + 1][j + 1]}{matrix[i][j + 2]}";

            return (upToBottom == "MAS" || upToBottom == "SAM") && (bottomToUp == "MAS" || bottomToUp == "SAM");
        }

        private static int PartOne(List<List<char>> matrix)
        {
            int m = matrix.Count;
            int n = matrix[0].Count;

            int count = 0;

            for (int i = 0; i < m; i++)
            {
                for (int j = 0; j < n; j++)
                {
                    if (matrix[i][j] == 'X')
                    {
                        // directions:
                        // 0: left, 1: left-down, 2: down
                        // 3: right-down, 4: right, 5: right-up
                        // 6: up, 7: left-up
                        for (int dir = 0; dir < 8; dir++)
                        {
                            count += CountXmas(matrix, i, j, 'X', dir);
                        }
                    }
                }
            }

            return count;
        }

        private static int CountXmas(
            List<List<char>> matrix,
            int i,
            int j,
            char currChar,
            int direction
        )
        {
            int m = matrix.Count;
            int n = matrix[0].Count;

            // base case
            if (i >= m || j >= n || i < 0 || j < 0 || matrix[i][j] != currChar)
            {
                return 0;
            }

            if (currChar == 'S')
            {
                // Console.WriteLine($"{i}, {j}");
                return 1;
            }

            // recursive case
            int count = 0;
            char nextChar = GetNextChar(currChar);

            if ((currChar == 'X' || currChar == 'M' || currChar == 'A') && nextChar != 'E')
            {
                switch (direction)
                {
                    case 0:
                        count += CountXmas(matrix, i, j - 1, nextChar, 0);
                        break;
                    case 1:
                        count += CountXmas(matrix, i + 1, j - 1, nextChar, 1);
                        break;
                    case 2:
                        count += CountXmas(matrix, i + 1, j, nextChar, 2);
                        break;
                    case 3:
                        count += CountXmas(matrix, i + 1, j + 1, nextChar, 3);
                        break;
                    case 4:
                        count += CountXmas(matrix, i, j + 1, nextChar, 4);
                        break;
                    case 5:
                        count += CountXmas(matrix, i - 1, j + 1, nextChar, 5);
                        break;
                    case 6:
                        count += CountXmas(matrix, i - 1, j, nextChar, 6);
                        break;
                    case 7:
                        count += CountXmas(matrix, i - 1, j - 1, nextChar, 7);
                        break;

                    default:
                        throw new ArgumentException("direction should be between 0 and 7");
                }
            }

            return count;
        }

        private static char GetNextChar(char c)
        {
            return c switch
            {
                'X' => 'M',
                'M' => 'A',
                'A' => 'S',
                _ => 'E',// end
            };
        }
    }
}
