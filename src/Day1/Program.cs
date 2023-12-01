using Day1;

Console.WriteLine(File.ReadLines(Environment.CurrentDirectory + "/input.txt").Sum(PartTwoSolve));

int PartOneSolve(string s)
{
    var digits = s
        .Select(x =>
        {
            var success = int.TryParse(new ReadOnlySpan<char>(x), out var value);
            return new { success, value };
        })
        .Where(x => x.success)
        .Select(x => x.value)
        .ToList();

    if (!int.TryParse($"{digits.First()}{digits.Last()}", out var rowResult))
    {
        throw new InvalidOperationException();
    }

    return rowResult;
}

int PartTwoSolve(string s)
{
    var digits = s
        .ReplaceStringNumbers()
        .Select(x =>
        {
            var success = int.TryParse(new ReadOnlySpan<char>(x), out var value);
            return new { success, value };
        })
        .Where(x => x.success)
        .Select(x => x.value)
        .ToList();

    if (!int.TryParse($"{digits.First()}{digits.Last()}", out var rowResult))
    {
        throw new InvalidOperationException();
    }

    return rowResult;
}