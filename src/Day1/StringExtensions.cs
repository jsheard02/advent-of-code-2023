namespace Day1;

public static class StringExtensions
{
    private static readonly Dictionary<string, string> Replacements = new()
    {
        { "one", "o1e" },
        { "two", "t2o" },
        { "three", "t3e" },
        { "four", "f4r" },
        { "five", "f5e" },
        { "six", "s6x" },
        { "seven", "s7n" },
        { "eight", "e8t" },
        { "nine", "n9e" },
    };

    public static string ReplaceStringNumbers(this string s) => 
        Replacements.Aggregate(s, (current, replacement) => current.Replace(replacement.Key, replacement.Value));
}