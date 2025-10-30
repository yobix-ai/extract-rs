# Tika 3.2.3 + GraalVM 25 Upgrade Notes

**Date**: 2025-10-30
**Branch**: feat/upgrade-tika-3.2.3-graalvm-25
**Purpose**: Upgrade from Tika 2.9.2 (EOL) to 3.2.3 (current stable)

---

## Changes Made

### 1. Tika Version Upgrade
- **From**: 2.9.2 (EOL April 2025)
- **To**: 3.2.3 (Latest stable, October 2025)
- **File**: extractous-core/tika-native/build.gradle:7

### 2. Logging Dependencies Update
- **slf4j-nop**: 2.0.11 → 2.0.16
- **log4j-to-slf4j**: 3.0.0-beta2 → 3.0.0 (stable)
- **File**: extractous-core/tika-native/build.gradle:31-34

### 3. GraalVM Version Upgrade
- **From**: GraalVM 23
- **To**: GraalVM 25
- **File**: extractous-core/tika-native/build.gradle:94

### 4. GraalVM 25 Optimization Flags
Added new flags for better performance:
- `--strict-image-heap`: Better memory layout
- `-H:+UnlockExperimentalVMOptions`: Access to experimental features
- `-H:+UseCompressedReferences`: Reduced memory footprint
- `-H:+RemoveUnusedSymbols`: Smaller binary size
- `-H:+ReportExceptionStackTraces`: Better debugging

---

## Next Steps

### Before Building:
1. Install GraalVM 25:
   ```bash
   sdk install java 25.0.0-graalce
   sdk use java 25.0.0-graalce
   ```

2. Verify installation:
   ```bash
   java -version  # Should show GraalVM CE 25
   native-image --version  # Should show 25.0.x
   ```

### Build Process:
1. Clean build:
   ```bash
   cd extractous-core/tika-native
   ./gradlew clean
   ```

2. Test dependency resolution:
   ```bash
   ./gradlew dependencies | grep tika
   # All should show 3.2.3
   ```

3. Run with tracing agent (regenerate metadata):
   ```bash
   ./gradlew -Pagent run
   ./gradlew -Pagent test
   ```

4. Build native library:
   ```bash
   ./gradlew nativeCompile
   ```

5. Test native library:
   ```bash
   ./gradlew nativeTest
   ```

### Metadata Regeneration Required:
- [ ] Linux: reachability-metadata.json
- [ ] Windows: reachability-metadata.json
- [ ] macOS: reachability-metadata.json

Old metadata locations:
- `src/main/resources/META-INF/ai.yobix/tika-2.9.2-*/`

New metadata locations (to be created):
- `src/main/resources/META-INF/ai.yobix/tika-3.2.3-*/`

---

## Testing Checklist

### Format Coverage:
- [ ] PDF
- [ ] DOCX, DOC
- [ ] XLSX, XLS
- [ ] PPTX, PPT
- [ ] EML, MSG, PST
- [ ] ZIP, TAR, 7Z
- [ ] WordPerfect, Lotus 1-2-3
- [ ] Encrypted PDF
- [ ] Password-protected Office

### Platforms:
- [ ] Linux (Ubuntu 22.04+)
- [ ] Windows (Windows 10/11)
- [ ] macOS (Intel + M1/M2)

### Performance:
- [ ] Benchmark vs 2.9.2 version
- [ ] Memory usage profiling
- [ ] Startup time measurement

---

## Expected Issues & Solutions

### Issue 1: Tika 3.x API Changes

**Symptoms**: Compilation errors in TikaNativeMain.java

**Solution**: Review Tika 3.x migration guide:
- https://github.com/apache/tika/blob/main/CHANGES.txt

Common changes:
- PDFParserConfig constructor
- OfficeParserConfig methods
- Deprecated API removals

### Issue 2: GraalVM Metadata Incomplete

**Symptoms**: Runtime errors like "class X is not accessible"

**Solution**: Re-run tracing agent with comprehensive test coverage

### Issue 3: Platform-Specific Compilation Failures

**Symptoms**: native-image fails on Windows/macOS

**Solution**: Platform-specific metadata may need adjustment

---

## PR Submission

Once testing complete:

```bash
git add .
git commit -m "feat: Upgrade to Tika 3.2.3 and GraalVM 25

- Upgrade Apache Tika 2.9.2 → 3.2.3 (Tika 2.x EOL April 2025)
- Upgrade GraalVM requirement 23 → 25
- Update logging to stable versions (remove beta)
- Add GraalVM 25 optimization flags
- Regenerate native-image reachability metadata

BREAKING CHANGES: None (internal version bump only)

Tested on Linux/Windows/macOS with comprehensive format coverage.
"

git push origin feat/upgrade-tika-3.2.3-graalvm-25

gh pr create --fill
```

---

**Status**: Ready for build and testing
**Owner**: ai-suite team
**Upstream PR**: To be submitted after validation
