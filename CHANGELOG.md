# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) (post version 0.2.0),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Version 0.2.0 (05/06/2026)

### Added

- Added the int_inc_dec_ext_tests test module with opp_omm_test_1, opp_omm_test_2, wpp_wmm_test_1 and wpp_wmm_test_2 test functions.

- Added "Paul Saunders" to the package authors field.

- Added the num_tests and int_num_tests test modules to the num module.

- Added the non_zero_i8_test and non_zero_u8_test test functions to the num::int_num_tests module.

- Added the non_zero_i8_test and non_zero_u8_test test functions to the num::int_num_tests module.

- Added the non_zero_unsigned_opp, non_zero_wpp, non_zero_signed_wmm and non_zero_unsigned_wmm macros.

- Added an IntIncDecExt implementation for the core::num::NonZeroI16 type in the num module.

- Added an IntIncDecExt implementation for the core::num::NonZeroI32 type in the num module.

- Added an IntIncDecExt implementation for the core::num::NonZeroI64 type in the num module.

- Added an IntIncDecExt implementation for the core::num::NonZeroI128 type in the num module.

- Added an IntIncDecExt implementation for the core::num::NonZeroIsize type in the num module.

- Added an IntIncDecExt implementation for the core::num::NonZeroU8 type in the num module.

- Added an IntIncDecExt implementation for the core::num::NonZeroU16 type in the num module.

- Added an IntIncDecExt implementation for the core::num::NonZeroU32 type in the num module.

- Added an IntIncDecExt implementation for the core::num::NonZeroU64 type in the num module.

- Added an IntIncDecExt implementation for the core::num::NonZeroU128 type in the num module.

- Added an IntIncDecExt implementation for the core::num::NonZeroUsize type in the num module.

- Added an IncDecExt implementation for the core::num::NonZeroU32 type in the num module.

- Added the non_zero_signed_opp, non_zero_signed_omm and non_zero_unsigned_omm macros.

- Added the non_zero_pp, non_zero_try_pp, non_zero_signed_mm, non_zero_signed_try_mm, non_zero_unsigned_mm and non_zero_unsigned_try_mm macros.

- Added an IncDecExt implementation for the core::num::NonZeroI16 type in the num module.

- Added an IncDecExt implementation for the core::num::NonZeroI32 type in the num module.

- Added an IncDecExt implementation for the core::num::NonZeroI64 type in the num module.

- Added an IncDecExt implementation for the core::num::NonZeroI128 type in the num module.

- Added an IncDecExt implementation for the core::num::NonZeroIsize type in the num module.

- Added an IncDecExt implementation for the core::num::NonZeroU8 type in the num module.

- Added an IncDecExt implementation for the core::num::NonZeroU16 type in the num module.

- Added an IncDecExt implementation for the core::num::NonZeroU64 type in the num module.

- Added an IncDecExt implementation for the core::num::NonZeroU128 type in the num module.

- Added an IncDecExt implementation for the core::num::NonZeroUsize type in the num module.

- Added an IntIncDecExt implementation for the core::num::NonZeroI8 type in the num module.

- Added the num feature.

- Added the num module and made it depend on the num feature.

- Added an IncDecExt implementation for the core::num::NonZeroI8 type in the num module.

- Added the GitHub funding file and updated the gitignore file.

- Added the "method" keyword to the package keywords field.



### Changed

- Updated the readme

- Renamed the following test functions; try_pp to test_pp_, try_ppf to test_ppf_, try_mm to test_mm_ and try_mmf to test_mmf_ in the inc_dec_ext_tests test module.

- Renamed the test module to inc_dec_ext_tests.

- Updated documentation

- Updated the MIT Licence copyright year.

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

- Renamed the IncDecSelf trait to IncDecExt and updated the relevant parts of the project accordingly.

- Renamed the IntIncDecSelf trait to IntIncDecExt and updated the relevant parts of the project accordingly.

-- Moved the IntIncDecExt related functionality from the inc_dec_exts module to the int_inc_dec_exts module.

- Moved the IntIncDecExt related functionality from the inc_dec_exts module to the newly added int_inc_dec_exts module.

- Renamed the inc_dec module to inc_dec_exts.

- Moved the macros from the inc_dec module into the new macros module.

- Moved the tests from the inc_dec module into the new tests module.

- Moved the traits from the inc_dec module into the new traits module.

- Replaced “doc_auto_cfg” with “doc_cfg” in the lib file.



### Removed

- Removed the package categories field.

- Removed the "floating-point" keyword from the package keywords field.



## Version 0.1.0 (21/03/2025)

- Initial release


