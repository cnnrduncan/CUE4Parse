# Getting Started with CUE4Parse Rust Bindings

This guide will help you quickly get started with using CUE4Parse from Rust applications.

## Prerequisites

- **Rust** 1.70+ (install from [rustup.rs](https://rustup.rs/))
- **.NET 8** SDK (install from [Microsoft](https://dotnet.microsoft.com/download))
- **CMake** 3.10+ (for native libraries, optional)
- **Git** for cloning the repository

## Quick Start

### 1. Clone and Build

```bash
# Clone the repository
git clone https://github.com/FabianFG/CUE4Parse.git
cd CUE4Parse

# Build everything with the provided script
./build.ps1 -All
```

### 2. Create Your Rust Project

```bash
# Create a new Rust project
cargo new my-ue-parser
cd my-ue-parser

# Add CUE4Parse dependency to Cargo.toml
```

Add to your `Cargo.toml`:
```toml
[dependencies]
cue4parse-rs = { path = "../CUE4Parse/cue4parse-rs" }
serde_json = "1.0"
```

### 3. Basic Usage

Create `src/main.rs`:

```rust
use cue4parse_rs::{Provider, GameVersion, Result};

fn main() -> Result<()> {
    // Replace with your game's path and version
    let mut provider = Provider::new("/path/to/your/game/paks", GameVersion::UE5_3);
    
    // Add your game's encryption key (if needed)
    provider.add_key("", "0xYOUR_GAME_AES_KEY_HERE");
    
    // Add mappings for better parsing (optional)
    provider.set_mappings("/path/to/game_mappings.usmap");
    
    // List some packages
    let packages = provider.list_packages()?;
    println!("Found {} packages", packages.len());
    
    // Show first 5 packages
    for package in packages.iter().take(5) {
        println!("Package: {}", package);
    }
    
    Ok(())
}
```

### 4. Run Your Application

```bash
cargo run
```

## Common Game Examples

### Fortnite

```rust
use cue4parse_rs::{Provider, GameVersion};

let mut provider = Provider::new(
    "/path/to/Fortnite/FortniteGame/Content/Paks", 
    GameVersion::UE5_3
);
provider.add_key("", "0xYOUR_FORTNITE_KEY");
```

### Rocket League

```rust
use cue4parse_rs::{Provider, GameVersion};

let mut provider = Provider::new(
    "/path/to/RocketLeague/TAGame/CookedPCConsole", 
    GameVersion::UE4_27
);
// Rocket League doesn't use encryption
```

### Fall Guys

```rust
use cue4parse_rs::{Provider, GameVersion};

let mut provider = Provider::new(
    "/path/to/FallGuys/FallGuys_client_game_Data", 
    GameVersion::UE4_27
);
provider.add_key("", "0xYOUR_FALL_GUYS_KEY");
```

## Working with Assets

### List and Explore Packages

```rust
// Get all packages
let packages = provider.list_packages()?;

// Filter for specific types
let textures: Vec<_> = packages
    .iter()
    .filter(|p| p.contains("Texture") || p.contains("Material"))
    .collect();

println!("Found {} texture packages", textures.len());
```

### Load Package Details

```rust
// Load a specific package
let package = provider.load_package("GameAssets/Characters/Hero.uasset")?;

println!("Package: {}", package.name);
println!("Exports: {}", package.exports.len());

for export in &package.exports {
    println!("  {} ({})", export.name, export.class_name);
}
```

### Export Objects

```rust
// Export an object as JSON
let json = provider.export_object_json("GameAssets/Textures/Logo.Logo")?;

// Pretty print the JSON
println!("{}", serde_json::to_string_pretty(&json)?);

// Access specific properties
if let Some(width) = json["SizeX"].as_u64() {
    println!("Texture width: {}", width);
}
```

## Troubleshooting

### "CLI tool not found"

Make sure you've built the CLI tool:
```bash
dotnet build CUE4Parse.CLI --configuration Release
```

Or specify a custom path:
```rust
provider.set_executable_path("/custom/path/to/CUE4Parse.CLI.exe");
```

### "Process execution failed"

1. **Check .NET installation**: `dotnet --version`
2. **Verify game path**: Make sure the directory exists and contains `.pak` or `.uasset` files
3. **Check encryption keys**: Some games require specific AES keys
4. **Try different game version**: Start with the closest UE version

### "Parse failed"

1. **Add encryption keys**: Many modern games encrypt their assets
2. **Use mappings**: Download or create `.usmap` files for your game
3. **Check game version**: Make sure you're using the correct UE version

## Finding Encryption Keys

Encryption keys are usually found through:

1. **Community**: Check game modding communities and forums
2. **GitHub**: Search for "[game name] aes key"
3. **Modding tools**: Other modding tools often share keys
4. **Game updates**: Keys sometimes change with game updates

**Important**: Only use keys from games you legally own.

## Next Steps

- Check out the [examples](examples/) directory for more use cases
- Read the [API documentation](https://docs.rs/cue4parse-rs)
- Join the CUE4Parse community for help and discussion
- Contribute improvements back to the project

## Performance Tips

- **Cache results**: Package listing can be slow, cache when possible
- **Filter early**: Use specific paths instead of listing everything
- **Batch operations**: Group related operations together
- **Use native features**: Enable compression library support for better performance

Happy parsing! 🎮🦀
