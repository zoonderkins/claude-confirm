<template>
  <div class="section-list">
    <h3 v-if="sections.length > 0" class="section-title">選擇要應用的段落：</h3>

    <div
      v-for="(section, index) in sections"
      :key="index"
      class="section-item"
      :class="{ selected: isSelected(index) }"
      @click="toggleSection(index)"
    >
      <input
        type="checkbox"
        :checked="isSelected(index)"
        @click.stop="toggleSection(index)"
        class="section-checkbox"
      />
      <div class="section-content">
        <h4 class="section-item-title">{{ section.title }}</h4>
        <p class="section-item-content">{{ section.content }}</p>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, watch } from 'vue'

const props = defineProps({
  sections: {
    type: Array,
    default: () => []
  },
  modelValue: {
    type: Array,
    default: () => []
  }
})

const emit = defineEmits(['update:modelValue'])

const selected = ref(
  props.sections
    .map((s, i) => s.selected ? i : -1)
    .filter(i => i >= 0)
)

function isSelected(index) {
  return selected.value.includes(index)
}

function toggleSection(index) {
  const idx = selected.value.indexOf(index)
  if (idx >= 0) {
    selected.value.splice(idx, 1)
  } else {
    selected.value.push(index)
  }
  emit('update:modelValue', selected.value)
}

watch(() => props.sections, (newSections) => {
  selected.value = newSections
    .map((s, i) => s.selected ? i : -1)
    .filter(i => i >= 0)
  emit('update:modelValue', selected.value)
}, { immediate: true })
</script>

<style scoped>
.section-list {
  margin: 1rem 0;
}

.section-title {
  font-size: 1.1rem;
  margin-bottom: 0.75rem;
  font-weight: 600;
}

.section-item {
  display: flex;
  align-items: flex-start;
  padding: 1rem;
  margin: 0.5rem 0;
  border: 2px solid var(--border-color, #e5e7eb);
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s ease;
  background: var(--bg-primary, white);
}

.section-item:hover {
  border-color: var(--accent-color, #9333EA);
  box-shadow: 0 2px 8px rgba(147, 51, 234, 0.15);
}

.section-item.selected {
  border-color: var(--accent-color, #9333EA);
  background: var(--accent-light, rgba(147, 51, 234, 0.1));
}

.section-checkbox {
  margin-right: 0.75rem;
  margin-top: 0.25rem;
  width: 18px;
  height: 18px;
  cursor: pointer;
  flex-shrink: 0;
}

.section-content {
  flex: 1;
  min-width: 0;
}

.section-item-title {
  font-size: 1rem;
  font-weight: 600;
  margin: 0 0 0.5rem 0;
  color: var(--text-primary, #1f2937);
  word-wrap: break-word;
}

.section-item-content {
  margin: 0;
  color: var(--text-secondary, #6b7280);
  font-size: 0.9rem;
  line-height: 1.5;
  word-wrap: break-word;
}

/* Responsive Design - Tablet */
@media (max-width: 768px) {
  .section-title {
    font-size: 1rem;
  }

  .section-item {
    padding: 0.875rem;
  }

  .section-checkbox {
    margin-right: 0.625rem;
    width: 16px;
    height: 16px;
  }

  .section-item-title {
    font-size: 0.95rem;
  }

  .section-item-content {
    font-size: 0.85rem;
  }
}

/* Responsive Design - Mobile */
@media (max-width: 480px) {
  .section-list {
    margin: 0.75rem 0;
  }

  .section-title {
    font-size: 0.95rem;
    margin-bottom: 0.5rem;
  }

  .section-item {
    padding: 0.75rem;
    margin: 0.375rem 0;
  }

  .section-checkbox {
    margin-right: 0.5rem;
  }

  .section-item-title {
    font-size: 0.9rem;
    margin-bottom: 0.375rem;
  }

  .section-item-content {
    font-size: 0.8rem;
    line-height: 1.4;
  }
}
</style>
