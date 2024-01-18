## libamicontained

A library for reasoning about the current process' resource constraints.

Mostly this is a straw man repo for code that may eventually live in
[libresource][libresource], to be discussed at [FOSDEM][fosdem].

## Design goals

`libamicontained` (or something like it) should be: small, fast, correct. To
enforce correctness, we have chosen rust. We have deliberately not added any
dependencies to keep it small (although the .a file is currently quite large,
and that needs to be fixed), and hopefully it is fast.

## Contributing

Please sign all commits must include a `Signed-off-by:` line in their
description. This indicates that you certify [the following statement, known as
the Developer Certificate of Origin][dco]). You can automatically add this line
to your commits by using `git commit -s --amend`.

[dco]: https://developercertificate.org/
[libresource]: https://github.com/lxc/libresource
[fosdem]: https://fosdem.org/2024/schedule/event/fosdem-2024-3033-libamicontained-a-low-level-library-for-reasoning-about-resource-restriction/
