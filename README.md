# Project Masterpass 
(working title)

> **Attention! This project is still under heavy development, and SHOULD NOT be used in practice, as the algorithms could *change at any time without prior announcement*! The algorithm hasn't been audited, and should be used on your own volition! The author SHALL NOT be held liable for any damages arising from the use or misuse of this algorithm or any implementation of it (see Limitation of Liability section)!**

## Summary

Project Masterpass is a deterministic databaseless key management algorithm, aimed to provide a solution for people who cannot afford to store and protect the integrity and secrecy of their most crucial encryption keys, like journalists and whistleblowers.

For the first step (Master Key derivation), it utilises Argon2d for multiple rounds of strong GPU resistant key derivation with service specific salts (aimed to protect the secrecy of the service keys, in case of the Master Passphrase being compromised), then it sends the output to HKDF, which then combined with service specific (assumed to be public) information derives a Service Key, which can be encoded in multiple ways, as required (as of now, the only encoding algorithm implemented is the EFF diceware wordlist (see the [Specification](spec.md)), but in the future multiple encoding algorithms are planned to be implemented, such as BIP-39).

See the [Specification](spec.md) for more details.

The reference implementation in Rust is provided for v1 (see spec) with **INSECURE default parameters optimized for speed for demo purposes ONLY!!!**. 


## Usage

```cli
cargo run
```

## Limitation of Liability

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.