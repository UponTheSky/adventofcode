using System.ComponentModel.DataAnnotations.Schema;
using System.Text;

namespace CSharp.Day2
{
    internal class Solution(string file) : CSharp.BaseSolution(file)
    {
        public override int Run()
        {
            IEnumerable<string> lines = ReadFrom(file);
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
    }
}
