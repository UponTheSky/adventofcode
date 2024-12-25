namespace CSharp
{
    internal abstract class BaseSolution(string file)
    {
        protected readonly string file = file;

        public abstract int Run();

        protected static IEnumerable<string> ReadFrom(string file)
        {
            string? line;

            using (var reader = File.OpenText(file))
            {
                while ((line = reader.ReadLine()) != null)
                {
                    yield return line;
                }
            }
        }
    }
}