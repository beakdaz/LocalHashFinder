using System;
using System.IO;
using System.Linq;
using System.Reflection;
using System.Text;

class SwiftyStringDumper
{
    static int Main(string[] args)
    {
        var exe = args.Length > 0
            ? args[0]
            : @"G:\[CRACKED.ST] SwiftyULP\[CRACKED.ST] SwiftyULP\SwiftyULP.exe";

        if (!File.Exists(exe))
        {
            Console.Error.WriteLine("exe not found: " + exe);
            return 1;
        }

        Assembly asm;
        try
        {
            asm = Assembly.LoadFrom(Path.GetFullPath(exe));
        }
        catch (Exception ex)
        {
            Console.Error.WriteLine("load failed: " + ex.Message);
            return 1;
        }

        Type[] types;
        try
        {
            types = asm.GetTypes();
        }
        catch (ReflectionTypeLoadException ex)
        {
            types = ex.Types.Where(t => t != null).ToArray();
            foreach (var le in ex.LoaderExceptions)
                if (le != null) Console.Error.WriteLine("loader: " + le.Message);
        }

        var type = types.FirstOrDefault(t => t.GetMethods(BindingFlags.Static | BindingFlags.NonPublic | BindingFlags.Public)
            .Any(m => m.Name == "nmKikxo27T"));
        if (type == null)
        {
            Console.Error.WriteLine("type with nmKikxo27T not found");
            return 1;
        }

        var method = type.GetMethod("nmKikxo27T", BindingFlags.Static | BindingFlags.NonPublic | BindingFlags.Public);
        if (method == null)
        {
            Console.Error.WriteLine("nmKikxo27T not found on " + type.FullName);
            return 1;
        }

        var offsetsFile = args.Length > 1 ? args[1] : null;
        var offsets = new System.Collections.Generic.List<int>();

        if (offsetsFile != null && File.Exists(offsetsFile))
        {
            int offParsed;
            foreach (var line in File.ReadAllLines(offsetsFile))
            {
                if (int.TryParse(line.Trim(), out offParsed))
                    offsets.Add(offParsed);
            }
        }
        else
        {
            for (int i = 0; i < 200000; i += 4)
                offsets.Add(i);
        }

        var results = new System.Collections.Generic.SortedDictionary<int, string>();
        foreach (var off in offsets)
        {
            try
            {
                var s = method.Invoke(null, new object[] { off }) as string;
                if (!string.IsNullOrEmpty(s) && s.Length >= 2 && s.Length < 500)
                {
                    bool ok = true;
                    foreach (var c in s)
                    {
                        if (char.IsControl(c) && c != '\t') { ok = false; break; }
                    }
                    if (ok) results[off] = s;
                }
            }
            catch
            {
            }
        }

        Console.WriteLine("decoded strings: " + results.Count);
        foreach (var kv in results)
            Console.WriteLine(string.Format("@{0:x}\t{1}", kv.Key, kv.Value));

        var outPath = Path.Combine(Path.GetDirectoryName(exe) ?? ".", "swifty_decoded_strings.txt");
        File.WriteAllLines(outPath, results.Select(kv => string.Format("{0:x}\t{1}", kv.Key, kv.Value)), Encoding.UTF8);
        Console.WriteLine("wrote " + outPath);
        return 0;
    }
}
