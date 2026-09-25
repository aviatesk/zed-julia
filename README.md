# aviatesk/zed-julia (archived)

> [!important]
> This repository is archived and no longer maintained. Use the official Zed
> extension for Julia,
> [JuliaEditorSupport/zed-julia](https://github.com/JuliaEditorSupport/zed-julia),
> instead.

This repository was a fork of
[JuliaEditorSupport/zed-julia](https://github.com/JuliaEditorSupport/zed-julia)
for testing the [JETLS](https://github.com/aviatesk/JETLS.jl) language server
in [Zed](https://zed.dev/). Since v0.2.0, the official extension uses JETLS as
its default language server
([zed-industries/extensions#7455](https://github.com/zed-industries/extensions/pull/7455)),
so this fork is no longer needed.

## Migrating to the official extension

If you have installed this fork as a dev extension, switch to the official
extension as follows:

1. Run the `zed: extensions` command, find the `Julia - JETLS` dev extension,
   and click `Uninstall`. Both extensions provide the Julia language, so keep
   only the official one installed.
2. In the same view, search for `julia`, select the extension named `Julia`,
   and click the install button.
3. Rename the `lsp.JETLS` section of your Zed settings to `lsp.jetls`, since
   the official extension registers the language server as `jetls`:

   ```jsonc
   {
     "lsp": {
       "jetls": { // previously "JETLS"
         "settings": {
           // ...
         },
         "initialization_options": {
           // ...
         },
       },
     },
   }
   ```

The official extension also differs from this fork in the following ways:

- It requires Julia v1.12.2 through 1.13.x, and
  [installs and updates the pinned JETLS release automatically](https://github.com/JuliaEditorSupport/zed-julia#automatic-installation-and-updates)
  in an extension-private depot. The `jetls` executable in `~/.julia/bin` is
  no longer used by the extension, but you can keep it for the `jetls` CLI
  (e.g. `jetls check`) or launch it explicitly via
  [`binary.path`](https://github.com/JuliaEditorSupport/zed-julia#custom-jetls-command).
- Custom `binary.arguments` spell out the full `julia` invocation, including
  `-m JETLS serve`, instead of arguments to the `jetls` executable. To run
  JETLS from a local checkout, pass `--project=/path/to/JETLS` in these
  arguments and leave `binary.path` unset; see
  [Launch configuration](https://github.com/JuliaEditorSupport/zed-julia#launch-configuration).
- The TestRunner.jl tasks of this fork are not included. Use the
  [TestRunner integration of JETLS](https://aviatesk.github.io/JETLS.jl/release/testrunner/)
  instead, which runs tests via code lenses and code actions.

See the
[official extension's README](https://github.com/JuliaEditorSupport/zed-julia#readme)
for the complete setup and configuration instructions.
