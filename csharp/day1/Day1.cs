using System.Text.RegularExpressions;

namespace CSharp.Day1
{
    internal class Solution(string file) : CSharp.BaseSolution(file)
    {
        public override int Run()
        {
            IEnumerable<string> lines = ReadFrom(file);
            Regex regex = new(pattern: @"^(\d+)\s+(\d+)$", options: RegexOptions.Compiled);

            List<int> firstArray = [];
            List<int> secondArray = [];

            foreach (string line in lines)
            {
                var match = regex.Match(line);

                if (match.Success)
                {
                    int first = Int32.Parse(match.Groups[1].Value);
                    int second = Int32.Parse(match.Groups[2].Value);

                    firstArray.Add(first);
                    secondArray.Add(second);
                }
            }

            // return PartOne(firstArray, secondArray);
            return PartTwo(firstArray, secondArray);

        }

        private static int PartOne(List<int> firstArray, List<int> secondArray)
        {
            firstArray.Sort();
            secondArray.Sort();

            return firstArray.Zip(secondArray, (first, second) => Math.Abs(second - first)).Sum();
        }

        private static int PartTwo(List<int> firstArray, List<int> secondArray)
        {
            Dictionary<int, int> intMap = [];

            foreach (int number in secondArray)
            {
                if (intMap.ContainsKey(number))
                {
                    intMap[number]++;
                }
                else
                {
                    intMap.Add(number, 1);
                }
            }

            int sum = 0;

            foreach (int number in firstArray)
            {
                if (intMap.TryGetValue(number, out int count))
                {
                    sum += number * count;
                }
            }

            return sum;
        }
    }
}
