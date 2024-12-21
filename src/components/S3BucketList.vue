<script setup lang="ts">
import { ref, onMounted } from "vue";

import { invoke } from "@tauri-apps/api/core";

import { AppStore } from "../lib/store";

const DEFAULT_PROFILE = "default";

const buckets = ref<string[]>([]);
const isLoading = ref(false);
const error = ref<string | null>(null);
const profiles = ref<string[]>([]);
const selectedProfile = ref<string>(DEFAULT_PROFILE);
const isLoadingProfiles = ref(true);

const loadProfiles = async () => {
  try {
    const awsProfiles = (await invoke("get_aws_profiles")) as string[];
    profiles.value = awsProfiles;

    // Load saved profile preference
    const savedProfile = await AppStore.getItem("selectedAwsProfile");
    if (savedProfile && profiles.value.includes(savedProfile as string)) {
      selectedProfile.value = savedProfile as string;
    }
  } catch (e) {
    error.value =
      e instanceof Error ? e.message : "Failed to load AWS profiles";
    console.error("Error loading profiles:", e);
  } finally {
    isLoadingProfiles.value = false;
  }
};

const saveProfileSelection = async (profile: string) => {
  try {
    await AppStore.setItem("selectedAwsProfile", profile);
  } catch (e) {
    console.error("Failed to save profile selection:", e);
  }
};

const fetchBuckets = async () => {
  isLoading.value = true;
  error.value = null;

  try {
    const output = (await invoke("list_s3_buckets", {
      profile: selectedProfile.value,
    })) as string;

    buckets.value = output
      .split("\n")
      .filter((line) => line.trim())
      .map((line) => {
        const parts = line.trim().split(/\s+/);
        return parts[2];
      });
  } catch (e) {
    error.value = e instanceof Error ? e.message : "Failed to list buckets";
    console.error("Error fetching buckets:", e);
  } finally {
    isLoading.value = false;
  }
};

const handleProfileChange = async (event: Event) => {
  const profile = (event.target as HTMLSelectElement).value;
  selectedProfile.value = profile;
  await saveProfileSelection(profile);
  await fetchBuckets();
};

onMounted(() => {
  loadProfiles();
});
</script>

<template>
  <div class="s3-buckets">
    <div class="controls">
      <div class="profile-selector">
        <label for="profile-select">AWS Profile:</label>
        <select
          id="profile-select"
          v-model="selectedProfile"
          @change="handleProfileChange"
          :disabled="isLoadingProfiles"
          class="profile-select"
        >
          <option v-if="isLoadingProfiles" value="">Loading profiles...</option>
          <option
            v-else
            v-for="profile in profiles"
            :key="profile"
            :value="profile"
          >
            {{ profile }}
          </option>
        </select>
      </div>

      <button
        @click="fetchBuckets"
        :disabled="isLoading || isLoadingProfiles"
        class="fetch-button"
      >
        {{ isLoading ? "Loading..." : "List S3 Buckets" }}
      </button>
    </div>

    <div v-if="error" class="error-message">
      {{ error }}
    </div>

    <div v-if="buckets.length > 0" class="bucket-list">
      <h3>S3 Buckets ({{ selectedProfile }}):</h3>
      <ul>
        <li v-for="bucket in buckets" :key="bucket">
          {{ bucket }}
        </li>
      </ul>
    </div>
  </div>
</template>

<style scoped>
.s3-buckets {
  padding: 1rem;
}

.controls {
  display: flex;
  gap: 1rem;
  align-items: center;
  margin-bottom: 1rem;
}

.profile-selector {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.profile-select {
  padding: 0.5rem;
  border: 1px solid #e5e7eb;
  border-radius: 4px;
  min-width: 200px;
}

.fetch-button {
  padding: 0.5rem 1rem;
}

.error-message {
  color: #dc2626;
  margin: 1rem 0;
  padding: 0.5rem;
  border-radius: 4px;
  background-color: #fef2f2;
}

.bucket-list {
  margin-top: 1rem;
}

.bucket-list ul {
  list-style: none;
  padding: 0;
}

.bucket-list li {
  padding: 0.5rem;
  border-bottom: 1px solid #e5e7eb;
}

.bucket-list li:last-child {
  border-bottom: none;
}
</style>
