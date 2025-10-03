<script setup lang="ts">
import { defineProps, computed } from 'vue'
import { parseMediaInfo, type MediaInfoParsed } from '@/services/fileinfo/mediainfo/parseMediInfo'
import MediaInfoSummary from './MediaInfoSummary.vue'

const props = defineProps<{
  mediainfo: string
  showPreview?: boolean
}>()

import { useI18n } from 'vue-i18n'
const { t } = useI18n()

const parsed = computed<MediaInfoParsed>(() => parseMediaInfo(props.mediainfo))
</script>
<template>
  <div class="media-info-preview">
    <div class="header-text">{{ t('torrent.mediainfo') }}</div>
    <MediaInfoSummary v-if="showPreview" :source="parsed" />
  </div>
</template>
<style scoped>
.media-info-preview {
    max-width: 98%;
}
.media-info-preview .header-text {
  font-weight: bold;
  font-size: 15px;
}
</style>
