commit 93ceec181db2d635d6e0294560195a065cb9d9d6 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Wed Jun 3 17:10:10 2026 +1200

    - Started work on the changelog.
    
    - Updated the readme
    
    - Renamed the following test functions; try_pp_ to test_pp_, try_ppf_ to test_ppf_, try_mm_ to test_mm_ and try_mmf_ to test_mmf_ in the inc_dec_ext_tests test module.

commit d80858195d6786e3ed38ffb946862ffe7e94dcda -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Wed Jun 3 16:10:33 2026 +1200

    - Updated the readme.
    
    - Renamed the test module to inc_dec_ext_tests.
    
    - Added the int_inc_dec_ext_tests test module with opp_omm_test_1, opp_omm_test_2, wpp_wmm_test_1 and wpp_wmm_test_2 test functions.

commit 1c4fa54a0bfffed3ef1fcc58a53abcb6789c8fd9 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Tue Jun 2 18:44:52 2026 +1200

    - Updated the package version to "0.2.0-beta".
    
    - Removed the package categories field.
    
    - Updated the reademe.
    
    - Updated documentation

commit 6a09265db734b4dd84924fac5223f568a5210d52 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Mon Jun 1 19:43:46 2026 +1200

    - Removed the "floating-point" keyword from the package keywords field.
    
    - Added "Paul Saunders" to the package authors field.
    
    - Updated the MIT Licence copyright year.
    
    - Updated the readme.
    
    - Updated documentation

commit a8c06af361077cf6683b57655505af39e55f197f -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Mon Jun 1 17:50:11 2026 +1200

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
    
    - Removed “_mut” from the all the macro definitions in the num module.

commit 5412d5e3ed8c48b3e60bc3e87f46a8e036ede84f -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Sat May 30 17:49:39 2026 +1200

    - Made the num module private.
    
    - Added the num_tests and int_num_tests test modules to the num module.
    
    - Added the non_zero_i8_test and non_zero_u8_test test functions to the num::int_num_tests module.
    
    - The non_zero_unsigned_mm_mut macro now panics if the provided value gets decremented to zero.
    
    - Added the non_zero_i8_test and non_zero_u8_test test functions to the num::int_num_tests module.

commit 070a0586f02ba5da7c563a591c1114107024638f -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Fri May 29 18:59:25 2026 +1200

    - Renamed the non_zero_opp_mut macro to non_zero_signed_opp_mut.
    
    - Added the non_zero_unsigned_opp_mut, non_zero_wpp_mut, non_zero_signed_wmm_mut and non_zero_unsigned_wmm_mut macros.
    
    - The NonZeroI8 implementation of IntIncDecExt now uses macros.
    
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
    
    Note, an earlier commit missed: - Added an IncDecExt implementation for the core::num::NonZeroU32 type.
    
    - Other minor changes.

commit 6db695f26d86fdf1304b6b73422e8267cbe928af -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Thu May 28 18:29:10 2026 +1200

    - Added the non_zero_opp_mut, non_zero_signed_omm_mut and non_zero_unsigned_omm_mut macros.
    
    - Removed a parameter from a lot of macros.
    
    - Fiddled with wrapping bounds.

commit 92d66e7986a56045cb4bb049bbeea3d94083922e -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Wed May 27 20:29:41 2026 +1200

    - Added the non_zero_pp_mut, non_zero_try_pp_mut, non_zero_signed_mm_mut, non_zero_signed_try_mm_mut, non_zero_unsigned_mm_mut and non_zero_unsigned_try_mm_mut macros.
    
    - The IncDecExt implementation of the core::num::NonZeroI8 type now uses the newly added macros in its method definitions.
    
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

commit c2ab34adbea3dfa7ab601bfb7c73ba0d57e90295 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Tue May 26 19:24:49 2026 +1200

    - Added the num feature.
    
    - Added the num module and made it depend on the num feature.
    
    - Added an IncDecExt implementation for the core::num::NonZeroI8 type.

commit 02695ae2e14ea7a782c6de42877b0ef60ed02219 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Tue May 26 15:49:20 2026 +1200

    - Renamed the IncDecSelf trait to IncDecExt and updated the relevant parts of the project accordingly.
    
    - Renamed the IntIncDecSelf trait to IntIncDecExt and updated the relevant parts of the project accordingly.
    
    - Moved the IntIncDecExt related functionality from the inc_dec_exts module to the int_inc_dec_exts module.
    
    - Removed the tests mod block but left its contents in the tests module file.

commit d8cb9af4ec303d93fe72e35bd2aec64c15e04fb7 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Tue May 26 14:56:09 2026 +1200

    - Renamed the inc_dec module to inc_dec_exts.

commit 7f33752103707993fbd07ce3937f80152f31e381 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Tue May 26 14:30:38 2026 +1200

    - Moved the macros from the inc_dec module into the new macros module.
    
    - Moved the tests from the inc_dec module into the new tests module.
    
    - Moved the traits from the inc_dec module into the new traits module.
    
    - Replaced “doc_auto_cfg” with “doc_cfg” in the lib file.

commit a4429f029ecb24a619ccae65034a5e21d67049ab -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Fri Mar 21 15:12:20 2025 +1300

    Added the GitHub funding file and updated the gitignore file.

commit 85366759fcd53532075de3bc0dfecf49edff4045 --
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Fri Mar 21 15:08:32 2025 +1300

    Initial commit
