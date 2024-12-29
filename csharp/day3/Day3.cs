using System.Text.RegularExpressions;

namespace CSharp.Day3
{
    internal class Solution(string file) : BaseSolution(file)
    {
        public override int Run()
        {
            IEnumerable<string> lines = ReadFrom(file);

            return PartTwo(lines);
        }

        private static int PartOne(IEnumerable<string> lines)
        {
            Regex regex = new(pattern: @"mul\((\d+)\,(\d+)\)", options: RegexOptions.Compiled);

            int totalSum = 0;

            foreach (string line in lines)
            {
                int partialSum = 0;
                var matches = regex.Matches(line);

                foreach (Match match in matches)
                {
                    int mul = 1;

                    for (int ctr = 1; ctr < match.Groups.Count; ctr++)
                    {
                        foreach (Capture capture in match.Groups[ctr].Captures)
                        {
                            mul *= Int32.Parse(capture.Value);
                        }
                    }

                    partialSum += mul;
                }

                totalSum += partialSum;
            }

            return totalSum;
        }


        private static int PartTwo(IEnumerable<string> lines)
        {
            Regex lineRegex = new(pattern: @"(mul\(\d+\,\d+\)|do\(\)|don't\(\))", options: RegexOptions.Compiled);
            Regex mulRegex = new(pattern: @"^mul\((\d+)\,(\d+)\)$", options: RegexOptions.Compiled);

            int totalSum = 0;

            List<string> ops = [];

            bool isEnabled = true;
            foreach (string line in lines)
            {
                List<string> matches = lineRegex.Matches(line).Select(match => match.Value).ToList();
                int partialSum = 0;

                foreach (var str in matches)
                {
                    if (str == "do()")
                    {
                        isEnabled = true;
                    }

                    if (str == "don't()")
                    {
                        isEnabled = false;
                    }

                    Match match = mulRegex.Match(str);

                    if (match.Success && isEnabled)
                    {
                        int mul = 1;

                        for (int ctr = 1; ctr < match.Groups.Count; ctr++)
                        {
                            foreach (Capture capture in match.Groups[ctr].Captures)
                            {
                                mul *= Int32.Parse(capture.Value);
                            }
                        }

                        partialSum += mul;
                    }
                }

                totalSum += partialSum;
                ops.Clear();
            }

            return totalSum;
        }
    }
}
