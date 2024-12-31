namespace CSharp
{
    internal class Program
    {
        static void Main(string[] args)
        {

            // #if DEBUG
            //             args = ["../../../day2/test.txt"];
            // #endif
            if (args.Length == 0)
            {
                Console.WriteLine("please provide the input file's path");
                return;
            }

            string file = args[0];
            Day4.Solution solution = new(file);

            Console.WriteLine(solution.Run());
        }

    }
}
