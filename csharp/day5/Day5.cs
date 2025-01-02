namespace CSharp.Day5
{
    using System.Globalization;
    using System.Text.RegularExpressions;
    using OrderRule = Dictionary<int, HashSet<int>>;
    using UpdateList = List<List<int>>;

    internal class Solution(string file) : BaseSolution(file)
    {
        public override int Run()
        {
            IEnumerable<string> lines = ReadFrom(file);

            var parsedInput = ParseInput(lines);

            OrderRule orderRule = parsedInput.Item1;
            UpdateList updates = parsedInput.Item2;

            return PartTwo(orderRule, updates);
        }

        private static Tuple<OrderRule, UpdateList> ParseInput(IEnumerable<string> lines)
        {
            Regex orderRegex = new(@"^(\d+)\|(\d+)$", RegexOptions.Compiled);
            OrderRule orderRule = [];

            // orders 
            foreach (var line in lines)
            {
                var match = orderRegex.Match(line);

                if (match.Success)
                {
                    int first = Int32.Parse(match.Groups[1].Captures[0].Value);
                    int second = Int32.Parse(match.Groups[2].Captures[0].Value);

                    if (!orderRule.ContainsKey(first))
                    {
                        orderRule.Add(first, []);
                    }

                    orderRule[first].Add(second);
                }
            }

            // updates
            lines = lines.SkipWhile(line => line != "");

            UpdateList updates = [];

            foreach (var line in lines)
            {
                if (line == "")
                {
                    continue;
                }

                updates.Add(line.Split(',').Select(Int32.Parse).ToList());
            }

            return Tuple.Create(orderRule, updates);
        }

        private static int PartOne(OrderRule orderRule, UpdateList updateList)
        {
            int sum = 0;

            foreach (var update in updateList)
            {
                if (IsValidUpdate(orderRule, update))
                {
                    sum += update[update.Count / 2];
                }
            }

            return sum;
        }

        private static int PartTwo(OrderRule orderRule, UpdateList updateList)
        {
            int sum = 0;

            foreach (var update in updateList)
            {
                if (!IsValidUpdate(orderRule, update))
                {
                    var validUpdate = GetValidUpdate(orderRule, update);
                    sum += validUpdate[validUpdate.Count / 2];
                }
            }

            return sum;

        }

        private static List<int> GetValidUpdate(OrderRule orderRule, List<int> update)
        {
            LinkedList<int> validUpdate = [];

            foreach (var num in update)
            {
                // loop over the linkedlist
                LinkedListNode<int>? node = validUpdate.First;
                bool addedToLast = true;

                while (node != null)
                {
                    if (orderRule.ContainsKey(num) && orderRule[num].Contains(node.Value))
                    {
                        validUpdate.AddBefore(node, num);
                        addedToLast = false;
                        break;
                    }

                    node = node.Next;
                }

                if (addedToLast)
                {
                    validUpdate.AddLast(num);
                }
            }

            return [.. validUpdate];
        }

        private static bool IsValidUpdate(OrderRule orderRule, List<int> update)
        {

            for (int i = 1; i < update.Count; i++)
            {
                if (orderRule.ContainsKey(update[i]))
                {
                    for (int j = 0; j < i; j++)
                    {
                        if (orderRule[update[i]].Contains(update[j]))
                        {
                            return false;
                        }
                    }
                }
            }

            return true;
        }
    }

}