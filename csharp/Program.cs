namespace CSharp
{
    internal class Program
    {
        static void Main(string[] args)
        {
            if (args.Length == 0)
            {
                Console.WriteLine("please provide the input file's path");
                return;
            }

            string file = args[0];
            Day1.Solution solution = new(file);

            Console.WriteLine(solution.Run());
        }

    }
}
