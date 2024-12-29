using System.ComponentModel.DataAnnotations.Schema;
using System.Diagnostics;
using System.Globalization;
using System.Linq.Expressions;
using System.Security.Cryptography;
using System.Text;

namespace CSharp.Day2
{
    internal class Solution(string file) : CSharp.BaseSolution(file)
    {
        public override int Run()
        {
            IEnumerable<string> lines = ReadFrom(file);

            return PartTwo(lines);
        }

        private static int PartOne(IEnumerable<string> lines)
        {
            int safeCount = 0;

            foreach (string line in lines)
            {
                List<string>? numbers = [.. line.Split(' ')];

                if (numbers != null && numbers.Count > 2)
                {
                    List<int> numbersInt = numbers.Select(Int32.Parse).ToList();

                    // first check 
                    int diff = numbersInt[1] - numbersInt[0];
                    if (diff == 0 || Math.Abs(diff) > 3)
                    {
                        continue;
                    }

                    bool isIncreasing = diff > 0;
                    bool isSafe = true;

                    // other checks
                    for (int i = 1; i < numbersInt.Count - 1; i++)
                    {
                        int curr = numbersInt[i];
                        int next = numbersInt[i + 1];
                        diff = numbersInt[i + 1] - numbersInt[i];

                        if (diff == 0 || Math.Abs(diff) > 3)
                        {
                            isSafe = false;
                            break;
                        }

                        if ((diff > 0) != isIncreasing)
                        {
                            isSafe = false;
                            break;
                        }
                    }

                    if (isSafe)
                    {
                        safeCount++;
                    }
                }

            }

            return safeCount;
        }

        private static int PartTwo(IEnumerable<string> lines)
        {
            int safeCount = 0;

            foreach (string line in lines)
            {
                List<string>? numbers = [.. line.Split(' ')];
                Debug.Assert(numbers != null && numbers.Count > 2);

                List<int> numbersInt = numbers.Select(Int32.Parse).ToList();
                int positiveCount = 0;
                int negativeCount = 0;

                for (int i = 0; i < numbersInt.Count - 1; i++)
                {
                    int diff = numbersInt[i + 1] - numbersInt[i];
                    if (diff > 0)
                    {
                        positiveCount++;
                    }
                    else if (diff < 0)
                    {
                        negativeCount++;
                    }
                }

                if (negativeCount < numbersInt.Count - 2 && positiveCount < numbersInt.Count - 2)
                {
                    continue;
                }

                if (negativeCount >= numbersInt.Count - 2)
                {
                    numbersInt.Reverse();
                }

                if (CheckValidArray(numbersInt))
                {
                    safeCount++;
                }
            }

            return safeCount;
        }

        private static bool CheckValidArray(List<int> numbers)
        {
            if (BruteForce(numbers))
            {
                return true;
            }

            for (int i = 0; i < numbers.Count; i++)
            {
                List<int> newNumbers = numbers.Where((_, index) => index != i).ToList();

                if (BruteForce(newNumbers))
                {
                    return true;
                }
            }

            return false;
        }

        private static bool BruteForce(List<int> numbers)
        {
            for (int i = 0; i < numbers.Count - 1; i++)
            {
                int diff = numbers[i + 1] - numbers[i];

                if (diff <= 0 || diff > 3)
                {
                    return false;
                }
            }

            return true;
        }
    }
}
