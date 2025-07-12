using System;
using System.Collections.Generic;
using System.CommandLine;
using System.IO;
using System.Linq;
using CUE4Parse.Encryption.Aes;
using CUE4Parse.FileProvider;
using CUE4Parse.MappingsProvider;
using CUE4Parse.UE4.Objects.Core.Misc;
using CUE4Parse.UE4.Versions;
using Newtonsoft.Json;
using Serilog;

namespace CUE4Parse.CLI
{
    public static class Program
    {
        public static int Main(string[] args)
        {
            // Configure logging to be minimal for CLI use
            Log.Logger = new LoggerConfiguration()
                .MinimumLevel.Warning()
                .WriteTo.Console()
                .CreateLogger();

            // Define CLI options
            var directoryOption = new Option<string>(
                "--directory",
                "Path to the game files directory"
            ) { IsRequired = true };

            var versionOption = new Option<string>(
                "--version",
                "Game version (e.g., GAME_UE5_3)"
            ) { IsRequired = true };

            var mappingsOption = new Option<string?>(
                "--mappings",
                "Path to mappings file (.usmap)"
            );

            var aesKeyOption = new Option<string[]>(
                "--aes-key",
                "AES key in format 'guid:key' (can be specified multiple times)"
            ) { AllowMultipleArgumentsPerToken = true };

            var listPackagesOption = new Option<bool>(
                "--list-packages",
                "List all available packages"
            );

            var packageOption = new Option<string?>(
                "--package",
                "Package path to load"
            );

            var packageInfoOption = new Option<bool>(
                "--package-info",
                "Get package information"
            );

            var objectOption = new Option<string?>(
                "--object",
                "Object path to export"
            );

            var exportOption = new Option<bool>(
                "--export",
                "Export object(s)"
            );

            var outputOption = new Option<string?>(
                "--output",
                "Output file path for exports"
            );

            var outputFormatOption = new Option<string>(
                "--output-format",
                () => "json",
                "Output format (json, png, fbx, etc.)"
            );

            // Create root command
            var rootCommand = new RootCommand("CUE4Parse CLI tool for Rust integration")
            {
                directoryOption,
                versionOption,
                mappingsOption,
                aesKeyOption,
                listPackagesOption,
                packageOption,
                packageInfoOption,
                objectOption,
                exportOption,
                outputOption,
                outputFormatOption
            };

            rootCommand.SetHandler(async (context) =>
            {
                try
                {
                    var directory = context.ParseResult.GetValueForOption(directoryOption)!;
                    var version = context.ParseResult.GetValueForOption(versionOption)!;
                    var mappings = context.ParseResult.GetValueForOption(mappingsOption);
                    var aesKeys = context.ParseResult.GetValueForOption(aesKeyOption) ?? Array.Empty<string>();
                    var listPackages = context.ParseResult.GetValueForOption(listPackagesOption);
                    var package = context.ParseResult.GetValueForOption(packageOption);
                    var packageInfo = context.ParseResult.GetValueForOption(packageInfoOption);
                    var objectPath = context.ParseResult.GetValueForOption(objectOption);
                    var export = context.ParseResult.GetValueForOption(exportOption);
                    var output = context.ParseResult.GetValueForOption(outputOption);
                    var outputFormat = context.ParseResult.GetValueForOption(outputFormatOption)!;

                    await ProcessCommand(directory, version, mappings, aesKeys, listPackages, 
                        package, packageInfo, objectPath, export, output, outputFormat);
                }
                catch (Exception ex)
                {
                    Console.Error.WriteLine($"Error: {ex.Message}");
                    context.ExitCode = 1;
                }
            });

            return rootCommand.Invoke(args);
        }

        private static async Task ProcessCommand(string directory, string version, string? mappings, 
            string[] aesKeys, bool listPackages, string? package, bool packageInfo, 
            string? objectPath, bool export, string? output, string outputFormat)
        {
            // Parse game version
            if (!Enum.TryParse<EGame>(version, out var gameVersion))
            {
                throw new ArgumentException($"Invalid game version: {version}");
            }

            // Create provider
            IFileProvider provider;
            if (directory.EndsWith(".apk"))
            {
                provider = new ApkFileProvider(directory, new VersionContainer(gameVersion));
            }
            else
            {
                provider = new DefaultFileProvider(directory, SearchOption.TopDirectoryOnly, true, new VersionContainer(gameVersion));
            }

            // Set mappings if provided
            if (!string.IsNullOrEmpty(mappings) && File.Exists(mappings))
            {
                provider.MappingsContainer = new FileUsmapTypeMappingsProvider(mappings);
            }

            // Initialize provider
            provider.Initialize();

            // Add AES keys
            foreach (var keyEntry in aesKeys)
            {
                var parts = keyEntry.Split(':', 2);
                if (parts.Length == 2)
                {
                    var guid = string.IsNullOrEmpty(parts[0]) ? new FGuid() : FGuid.Parse(parts[0]);
                    var key = new FAesKey(parts[1]);
                    provider.SubmitKey(guid, key);
                }
            }

            // Process commands
            if (listPackages)
            {
                var packages = provider.Files.Keys
                    .Where(x => x.EndsWith(".uasset") || x.EndsWith(".umap"))
                    .OrderBy(x => x)
                    .ToList();

                foreach (var pkg in packages)
                {
                    Console.WriteLine(pkg);
                }
            }
            else if (packageInfo && !string.IsNullOrEmpty(package))
            {
                var pkg = provider.LoadPackage(package);
                var exports = pkg.GetExports();

                var packageInfoObj = new
                {
                    name = package,
                    exports = exports.Select(export => new
                    {
                        name = export.Name,
                        class_name = export.Class?.Name ?? "Unknown",
                        outer_index = export.OuterIndex.Index
                    }).ToList()
                };

                Console.WriteLine(JsonConvert.SerializeObject(packageInfoObj, Formatting.None));
            }
            else if (export && !string.IsNullOrEmpty(objectPath))
            {
                if (outputFormat.ToLower() == "json")
                {
                    var obj = provider.LoadPackageObject(objectPath);
                    var json = JsonConvert.SerializeObject(obj, Formatting.None);
                    
                    if (!string.IsNullOrEmpty(output))
                    {
                        await File.WriteAllTextAsync(output, json);
                    }
                    else
                    {
                        Console.WriteLine(json);
                    }
                }
                else
                {
                    // For other formats, we'd need to implement specific exporters
                    // This is a placeholder for future export functionality
                    throw new NotImplementedException($"Export format '{outputFormat}' is not yet implemented");
                }
            }
            else
            {
                throw new ArgumentException("Invalid command combination. Use --help for usage information.");
            }
        }
    }
}
