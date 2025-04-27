# Mandelbrust

Mandelbrust is a command line application written in Rust which generates images of fractals with a variety of fractals, colors, resolutions, and options.

[Installation](#installation)

## Installation

### Build from Source

Make sure you have `cargo` installed.

#### Locally

Run the following commands to install `mandelbrust` using a local clone of the repository:

```bash
git clone https://github.com/EthanJ-Brady/mandelbrust.git
cd mandelbrust
cargo install --path .
```

#### From GitHub

Run the following command to install `mandelbrust` from the GitHub repository:

```bash
cargo install --git https://github.com/EthanJ-Brady/mandelbrust.git mandelbrust
```

After installation, the `mandelbrust` binary should be available in your Cargo bin path (`~/.cargo/bin/` by default).

### Nix

Make sure you have nix flakes enabled on your system.

#### Run Temporarily

Run the following command to run `mandelbrust` on your system without installing it permanently:

```bash
nix run github:EthanJ-Brady/mandelbrust -- --help
```

#### Install with Nix Profiles

```bash
nix profile install github:EthanJ-Brady/mandelbrust
```

#### Integrate with NixOS / Nix Darwin / Home Manager

**Add `mandelbrust` as an Input to Your Flake:**

Edit your system's main `flake.nix` (usually at the root of your configuration repository) and add `mandelbrust` to the `inputs`:

```nix
# /path/to/your/nix/config/flake.nix
{
  inputs = {
    # Your existing inputs (nixpkgs, home-manager, etc.)
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    home-manager.url = "github:nix-community/home-manager";
    # ... other inputs

    # Add mandelbrust
    mandelbrust = {
      url = "github:EthanJ-Brady/mandelbrust";
      # Optional: Follow nixpkgs input if you want consistency
      # inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, home-manager, ... }@inputs: {
    # Note: mandelbrust is implicitly in 'inputs'
    # Your system configurations (NixOS or Darwin)
    # Make sure to pass 'inputs' down, e.g., using specialArgs:
    # specialArgs = { inherit inputs; };
    # ...
  };
}
```

**Add the Package to Your Configuration:**
You can add the package either system-wide or via Home Manager. Edit the relevant `.nix` file within your configuration:

- **Option A: System-wide (`configuration.nix` for NixOS or `darwin-configuration.nix` for Darwin)**

  ```nix
  # In configuration.nix or darwin-configuration.nix
  { config, pkgs, inputs, ... }:
  # Ensure 'inputs' is passed (e.g., via specialArgs)
  {
    environment.systemPackages = with pkgs; [
      # Your other packages...
      inputs.mandelbrust.packages.${pkgs.system}.default
    ];
  }
  ```

- **Option B: Via Home Manager**
  ```nix
  # In home.nix
  { config, pkgs, inputs, ... }:
  # Ensure 'inputs' is passed (e.g., via extraSpecialArgs)
  {
    home.packages = with pkgs; [
      # Your other packages...
      inputs.mandelbrust.packages.${pkgs.system}.default
    ];
  }
  ```
