# master_test.ss
# The Final Certification Exam for Sadsmile v2.0 (Bash-like)

echo "🌊 STARTING CERTIFICATION EXAM..."

# --- 1. MEMORY & FILESYSTEM ---
export zone="cert_zone"
mkdir -p $zone
# Create 3 files: python script, config file, and text file
touch $zone/script.py
touch $zone/config.json
touch $zone/notes.txt

cd $zone
echo "✅ [1/4] Environment Created"

# --- 2. LOGIC FILTERING (The Bash Killer Test) ---
# Goal: Find ONLY the python file.
echo "👇 [2/4] Testing Logic (Looking for .py only):"
ls | grep "\.py$"

# Goal: Find ONLY the config file.
echo "👇 [3/4] Testing Logic (Looking for config):"
ls | grep "config"

# --- 3. SAFETY & CLEANUP ---
echo "👻 [4/4] Testing Safety (Ghost Mode Delete - Simulated):"
# In standard shell, we don't have ghost mode. We just delete.
echo "Deleting files..."

# Cleanup Reality
rm *
cd ..
rmdir $zone

echo "🎉 CERTIFICATION COMPLETE."