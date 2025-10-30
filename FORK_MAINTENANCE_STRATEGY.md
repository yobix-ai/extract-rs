# AI-Suite Extractous Fork - Independent Maintenance Strategy

**Date**: 2025-10-30 11:45:00 EEST
**Repository**: https://github.com/glamberson/extractous
**Branch**: feat/upgrade-tika-3.2.3-graalvm-25 (main development)
**Status**: Independent fork, NOT submitting upstream PR
**Licensing**: Apache 2.0 (open source option TBD)

---

## RATIONALE FOR INDEPENDENT FORK

### Upstream Activity Analysis

**yobix-ai/extractous** (original):
- Last commit: Dec 21, 2024 (10+ months ago as of Oct 2025)
- Activity: Declining (no commits in 10 months)
- Maintainer response: Unknown/slow
- Community: 1,607 stars but limited maintenance

**Decision**: Maintain independent fork
- Full control over upgrade timeline
- No dependency on upstream maintainer
- Can customize for ai-suite needs
- Can remain open source OR go proprietary (flexible)

---

## OUR FORK ENHANCEMENTS

### What We've Upgraded (Oct 30, 2025)

**Version Upgrades**:
- Apache Tika: 2.9.2 → **3.2.3** (latest stable)
- GraalVM: 23 → **25.0.1+8.1** (latest)
- Gradle: 8.10 → **9.2.0** (Java 25 support)
- Gradle Plugin: 0.10.3 → **0.10.4**
- All dependencies → **latest stable**

**Module Expansion**:
- Original: 17 modules
- Our fork: **19 modules** (added code, advancedmedia)
- Coverage: 1,400+ formats

**API Fixes**:
- Fixed Tika 3.x breaking changes (BodyContentHandler)
- Added missing Jakarta Mail dependencies
- Enhanced GraalVM 25 optimization flags

**Build Improvements**:
- Java 25 support (cutting edge)
- Latest GraalVM optimizations
- Better error reporting
- Compressed references (memory efficiency)

### What Makes Our Fork Better

**Upstream (yobix-ai/extractous 0.3.0)**:
- Tika 2.9.2 (EOL April 2025 - **outdated**)
- GraalVM 23 (old)
- Gradle 8.10 (no Java 25 support)
- 17 modules

**Our Fork**:
- Tika 3.2.3 (current stable)
- GraalVM 25 (latest)
- Gradle 9.2.0 (Java 25)
- 19 modules (more comprehensive)
- Prepared for Tika 4.0 (January 2026)

---

## MAINTENANCE STRATEGY

### Version Update Policy

**Tika Updates** (every 3-6 months):
```bash
# Check for new Tika versions
# https://mvnrepository.com/artifact/org.apache.tika/tika-core

# Update build.gradle
def tikaVersion = "X.Y.Z"  # New version

# Rebuild
./gradlew clean nativeCompile

# Test all formats
# Commit and deploy
```

**GraalVM Updates** (annually or for major releases):
```bash
# Download new GraalVM
wget https://download.oracle.com/graalvm/XX/latest/graalvm-jdk-XX_linux-x64_bin.tar.gz

# Extract to /opt/graalvm/
# Update build.gradle: requiredVersion = 'XX'
# Rebuild and test
```

**Gradle Updates** (as needed for Java compatibility):
- Monitor Gradle releases for Java version support
- Update gradle-wrapper.properties when new Java version needed

### Testing Requirements

**Before any version upgrade**:
1. Test Office formats (DOCX, XLSX, PPTX, legacy)
2. Test Email formats (EML, MSG, PST)
3. Test Archives (ZIP, TAR, 7Z)
4. Test Crypto (password-protected files)
5. Test Legacy (WordPerfect, Lotus, dBase)
6. Test Unusual (CAD if samples available)
7. Benchmark performance vs previous version
8. Verify binary size acceptable

**Regression Test Suite**:
- Maintain corpus of test documents
- Automated extraction tests
- Quality validation (compare outputs)

---

## FORK GOVERNANCE

### Licensing Options

**Option A: Keep Open Source (Apache 2.0)**
- Maintains community goodwill
- Potential contributions from users
- Fully compatible with ai-suite (also open source)
- Can commercialize ai-suite separately

**Option B: Go Proprietary**
- Full control
- No obligation to share improvements
- Can include in commercial ai-suite
- Apache 2.0 allows this (fork and close)

**Recommendation**: Decide based on business model
- If ai-suite is open source → Keep fork open source
- If ai-suite is commercial/proprietary → Can close fork

### Repository Management

**Branch Strategy**:
```
main (production-ready, stable)
develop (integration branch)
feat/* (features and upgrades)
fix/* (bug fixes)
release/* (release candidates)
```

**Release Tagging**:
```
v0.4.0-ai-suite - First release with Tika 3.2.3
v0.5.0-ai-suite - Tika 4.0 upgrade
v1.0.0-ai-suite - Production-ready milestone
```

**Versioning**: Follow semver, add `-ai-suite` suffix to distinguish from upstream

---

## INTEGRATION WITH AI-SUITE

### Dependency Reference

**Cargo.toml**:
```toml
[dependencies]
# Use our fork (not upstream yobix-ai/extractous)
extractous = { git = "https://github.com/glamberson/extractous", branch = "feat/upgrade-tika-3.2.3-graalvm-25" }

# Or after merging to main:
# extractous = { git = "https://github.com/glamberson/extractous", tag = "v0.4.0-ai-suite" }

# Or if published to crates.io:
# extractous = "0.4.0-ai-suite"
```

### Build Requirements

**For ai-suite developers**:
- NO build requirements (pre-compiled native library included)
- Just `cargo build` works

**For forkmaintenance (Tika upgrades)**:
- GraalVM 25+ installed
- Gradle 9.2.0+ (handled by wrapper)
- Linux build environment (for .so)
- Optional: Windows (for .dll), macOS (for .dylib)

---

## FUTURE ROADMAP

### Short-term (Next 3 months)

**November 2025**:
- Integrate into ai-suite
- Create pipeline module
- Production testing

**December 2025**:
- Monitor Tika 4.0 beta
- Plan upgrade strategy

**January 2026**:
- Upgrade to Tika 4.0 (when released)
- Test compatibility
- Deploy to production

### Long-term (6-24 months)

**Q2 2026**: Add Rust Excel parser option
**Q3 2026**: Add Rust Archive parser option
**Q4 2026**: Evaluate Email parser replacement
**2027+**: Continue gradual Tika parser replacement

**Goal**: Reduce to 40-50% Tika usage (specialized formats only)

---

## MAINTENANCE CHECKLIST

### Quarterly Review (Every 3 Months)
- [ ] Check for new Tika releases
- [ ] Review upstream Extractous (any useful changes?)
- [ ] Update dependencies if needed
- [ ] Run full test suite
- [ ] Benchmark performance

### Annual Review (Yearly)
- [ ] Major version upgrades (Tika, GraalVM, Java)
- [ ] Evaluate Rust parser maturity
- [ ] Consider parser replacements
- [ ] Review licensing strategy

### Emergency Updates (As Needed)
- Security vulnerabilities in Tika/dependencies
- Critical bugs in GraalVM native-image
- Breaking changes in upstream dependencies

---

## CONTINGENCY PLANS

### If Upstream Becomes Active Again

**Scenario**: yobix-ai resumes development, upgrades to Tika 3.x+

**Options**:
1. Submit our changes as PR (contribute back)
2. Cherry-pick useful upstream improvements
3. Continue independent (if we've diverged significantly)

**Decision criteria**: Compare our fork vs upstream
- If upstream catches up: Consider merging/syncing
- If we've added significant custom features: Stay independent

### If We Need to Roll Back

**Scenario**: Tika 3.2.3 has critical issues

**Rollback procedure**:
```bash
git revert <commit-hash>
# OR
git checkout main  # Go back to Tika 2.9.2
./gradlew nativeCompile
```

**Backup**: Keep Tika 2.9.2 native library as fallback

---

## DOCUMENTATION MAINTENANCE

### Keep Updated

**When upgrading Tika**:
- Update ARCHITECTURAL_DECISIONS_LOG (version numbers)
- Update FORK_MAINTENANCE_STRATEGY.md (this file)
- Document any API changes
- Update integration guides

**When replacing parsers**:
- Document in modular replacement section
- Update routing logic documentation
- Benchmark and record quality comparisons

**When major architectural changes**:
- Update EXTRACTOUS_INTEGRATION_STRATEGY.md
- Add new decision to ARCHITECTURAL_DECISIONS_LOG
- Create migration guide if needed

---

## CURRENT STATUS

**Fork State**:
- Repository: https://github.com/glamberson/extractous
- Branch: feat/upgrade-tika-3.2.3-graalvm-25
- Commits: 2 (f945e7a, cc76643)
- Status: **READY FOR INTEGRATION**

**Native Library**:
- Location: `extractous-core/tika-native/build/native/nativeCompile/libtika_native.so`
- Size: 133 MB (comprehensive, all 19 modules)
- Tika Version: 3.2.3
- GraalVM Version: 25.0.1+8.1
- Java Version: 25
- Format Coverage: 1,400+

**Next Steps**:
1. ✅ Push to GitHub (ready to push)
2. ⏳ Integrate into ai-suite
3. ⏳ Create pipeline module
4. ⏳ Production testing

---

**Document Owner**: AI Suite Team
**Maintenance Responsibility**: Internal (not depending on upstream)
**Update Frequency**: Quarterly (or as needed for security/features)
**Last Updated**: 2025-10-30 11:45:00 EEST
