# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) (post version 0.2.0),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Version 0.2.0 (__/06/2026)

### Added

commit d80858195d6786e3ed38ffb946862ffe7e94dcda

- Added the int_inc_dec_ext_tests test module with opp_omm_test_1, opp_omm_test_2, wpp_wmm_test_1 and wpp_wmm_test_2 test functions.

commit 6a09265db734b4dd84924fac5223f568a5210d52

- Added "Paul Saunders" to the package authors field.

commit 5412d5e3ed8c48b3e60bc3e87f46a8e036ede84f

- Added the num_tests and int_num_tests test modules to the num module.

- Added the non_zero_i8_test and non_zero_u8_test test functions to the num::int_num_tests module.

-- The non_zero_unsigned_mm_mut macro now panics if the provided value gets decremented to zero.

-- The non_zero_unsigned_mm macro now panics if the provided value gets decremented to zero.

Renamed - Added in this version.

- Added the non_zero_i8_test and non_zero_u8_test test functions to the num::int_num_tests module.

commit 070a0586f02ba5da7c563a591c1114107024638f

-- Added the non_zero_unsigned_opp_mut, non_zero_wpp_mut, non_zero_signed_wmm_mut and non_zero_unsigned_wmm_mut macros.

- Added the non_zero_unsigned_opp, non_zero_wpp, non_zero_signed_wmm and non_zero_unsigned_wmm macros.

- Added an IntIncDecExt implementation for the core::num::NonZeroI16 type.

- Added an IntIncDecExt implementation for the core::num::NonZeroI32 type.

- Added an IntIncDecExt implementation for the core::num::NonZeroI64 type.

- Added an IntIncDecExt implementation for the core::num::NonZeroI128 type.

- Added an IntIncDecExt implementation for the core::num::NonZeroIsize type.

- Added an IntIncDecExt implementation for the core::num::NonZeroU8 type.

- Added an IntIncDecExt implementation for the core::num::NonZeroU16 type.

- Added an IntIncDecExt implementation for the core::num::NonZeroU32 type.

- Added an IntIncDecExt implementation for the core::num::NonZeroU64 type.

- Added an IntIncDecExt implementation for the core::num::NonZeroU128 type.

- Added an IntIncDecExt implementation for the core::num::NonZeroUsize type.

-- Note, an earlier commit missed: - Added an IncDecExt implementation for the core::num::NonZeroU32 type.

- Added an IncDecExt implementation for the core::num::NonZeroU32 type.

commit 6db695f26d86fdf1304b6b73422e8267cbe928af

-- Added the non_zero_opp_mut, non_zero_signed_omm_mut and non_zero_unsigned_omm_mut macros.

- Added the non_zero_signed_opp, non_zero_signed_omm and non_zero_unsigned_omm macros.

Renamed

commit 92d66e7986a56045cb4bb049bbeea3d94083922e

-- Added the non_zero_pp_mut, non_zero_try_pp_mut, non_zero_signed_mm_mut, non_zero_signed_try_mm_mut, non_zero_unsigned_mm_mut and non_zero_unsigned_try_mm_mut macros.

- Added the non_zero_pp, non_zero_try_pp, non_zero_signed_mm, non_zero_signed_try_mm, non_zero_unsigned_mm and non_zero_unsigned_try_mm macros.

Renamed

- Added an IncDecExt implementation for the core::num::NonZeroI16 type.

- Added an IncDecExt implementation for the core::num::NonZeroI32 type.

- Added an IncDecExt implementation for the core::num::NonZeroI64 type.

- Added an IncDecExt implementation for the core::num::NonZeroI128 type.

- Added an IncDecExt implementation for the core::num::NonZeroIsize type.

- Added an IncDecExt implementation for the core::num::NonZeroU8 type.

- Added an IncDecExt implementation for the core::num::NonZeroU16 type.

- Added an IncDecExt implementation for the core::num::NonZeroU64 type.

- Added an IncDecExt implementation for the core::num::NonZeroU128 type.

- Added an IncDecExt implementation for the core::num::NonZeroUsize type.

- Added an IntIncDecExt implementation for the core::num::NonZeroI8 type.

commit c2ab34adbea3dfa7ab601bfb7c73ba0d57e90295

- Added the num feature.

- Added the num module and made it depend on the num feature.

- Added an IncDecExt implementation for the core::num::NonZeroI8 type.

commit a4429f029ecb24a619ccae65034a5e21d67049ab

- Added the GitHub funding file and updated the gitignore file.



### Changed

commit 93ceec181db2d635d6e0294560195a065cb9d9d6

-- Started work on the changelog.

- Updated the readme

-- Renamed the following test functions; try_pp_ to test_pp_, try_ppf_ to test_ppf_, try_mm_ to test_mm_ and try_mmf_ to test_mmf_ in the inc_dec_ext_tests test module.

- Renamed the following test functions; try_pp to test_pp_, try_ppf to test_ppf_, try_mm to test_mm_ and try_mmf to test_mmf_ in the inc_dec_ext_tests test module.

Renamed

commit d80858195d6786e3ed38ffb946862ffe7e94dcda

-- Updated the readme.

- Renamed the test module to inc_dec_ext_tests.

Added - Renamed

commit 1c4fa54a0bfffed3ef1fcc58a53abcb6789c8fd9

-- Updated the package version to "0.2.0-beta".

-- Updated the reademe.

- Updated documentation

commit 6a09265db734b4dd84924fac5223f568a5210d52

- Updated the MIT Licence copyright year.

-- Updated the readme.

-- Updated documentation

commit a8c06af361077cf6683b57655505af39e55f197f

- Renamed the pp macro to pp_.

- Renamed the pp_mut macro to pp and updated the relevant parts of the project accordingly.

- Renamed the checked_pp_mut macro to try_pp and updated the relevant parts of the project accordingly.

- Renamed the ppf macro to ppf_.

- Renamed the ppf_mut macro to ppf and updated the relevant parts of the project accordingly.

- Renamed the mm macro to mm_.

- Renamed the mm_mut macro to mm and updated the relevant parts of the project accordingly.

- Renamed the mmf macro to mmf_.

- Renamed the mmf_mut macro to mmf and updated the relevant parts of the project accordingly.

- Renamed the checked_mm_mut macro to try_mm and updated the relevant parts of the project accordingly.

- Renamed the opp_mut macro to opp and updated the relevant parts of the project accordingly.

- Renamed the omm_mut macro to omm and updated the relevant parts of the project accordingly.

- Renamed the wpp_mut macro to wpp and updated the relevant parts of the project accordingly.

- Renamed the wmm_mut macro to wmm and updated the relevant parts of the project accordingly.

-- Removed “_mut” from the all the macro definitions in the num module.

Added in this version.

commit 5412d5e3ed8c48b3e60bc3e87f46a8e036ede84f

-- Made the num module private.

commit 070a0586f02ba5da7c563a591c1114107024638f

-- Renamed the non_zero_opp_mut macro to non_zero_signed_opp_mut.

Renamed - Added in this version.

-- The NonZeroI8 implementation of IntIncDecExt now uses macros.

-- Other minor changes.

commit 6db695f26d86fdf1304b6b73422e8267cbe928af

-- Removed a parameter from a lot of macros.
    
-- Fiddled with wrapping bounds.

commit 92d66e7986a56045cb4bb049bbeea3d94083922e

-- The IncDecExt implementation of the core::num::NonZeroI8 type now uses the newly added macros in its method definitions.

commit 02695ae2e14ea7a782c6de42877b0ef60ed02219

- Renamed the IncDecSelf trait to IncDecExt and updated the relevant parts of the project accordingly.

- Renamed the IntIncDecSelf trait to IntIncDecExt and updated the relevant parts of the project accordingly.

-- Moved the IntIncDecExt related functionality from the inc_dec_exts module to the int_inc_dec_exts module.

- Moved the IntIncDecExt related functionality from the inc_dec_exts module to the newly added int_inc_dec_exts module.

Moved

commit d8cb9af4ec303d93fe72e35bd2aec64c15e04fb7

- Renamed the inc_dec module to inc_dec_exts.

commit 7f33752103707993fbd07ce3937f80152f31e381

- Moved the macros from the inc_dec module into the new macros module.

- Moved the tests from the inc_dec module into the new tests module.

- Moved the traits from the inc_dec module into the new traits module.

- Replaced “doc_auto_cfg” with “doc_cfg” in the lib file.



### Deprecated



### Removed

commit 1c4fa54a0bfffed3ef1fcc58a53abcb6789c8fd9

- Removed the package categories field.

commit 6a09265db734b4dd84924fac5223f568a5210d52

- Removed the "floating-point" keyword from the package keywords field.

commit 02695ae2e14ea7a782c6de42877b0ef60ed02219

-- Renamed the inc_dec module to inc_dec_exts.



### Fixed



### Security



## Version 0.1.0 (21/03/2025)

- Initial release


