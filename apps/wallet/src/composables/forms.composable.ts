import { computed, onMounted, Ref, ref, useSlots, useTemplateRef, watch } from 'vue';
import logger from '~/core/logger.core';
import { VFormValidation } from '~/types/helper.types';
import { deepClone, transformData } from '~/utils/helper.utils';

/**
 * A field with a list of error messages.
 */
export interface FieldWithError {
  field: string;
  errorMessages: string[];
  isCustomValidation?: boolean;
}

export const useForm = <T>(opts: {
  /**
   * The model to use for the form.
   */
  model: Ref<T>;
  /**
   * The submit function to call when the form is valid and submitted.
   *
   * @param model the model to submit
   * @returns
   */
  submit: (model: T) => Promise<void>;
  /**
   * Take a snapshot of the model for comparison.
   *
   * If not provided, the model will be stringified.
   */
  takeModelSnapshot?: (model: T) => string;
  /**
   * The form template ref to use for validation.
   *
   * If not provided, defaults to 'form'.
   */
  formTemplateRef?: string;
  /**
   * Additional validation rules to apply to the form.
   *
   * If not provided, only the form's built-in validation will be used.
   */
  isValid?: (model: T) => Promise<FieldWithError[]> | FieldWithError[];
}) => {
  useSlots;
  const form = useTemplateRef<VFormValidation>(opts.formTemplateRef ?? 'form');
  const valid = ref(true);
  const fieldsWithErrors = ref<Array<FieldWithError>>([]);
  const modelSnapshot = ref<string>('');
  const submitting = ref(false);
  const edited = ref(false);
  const submitted = ref(false);
  const initialModel = ref(deepClone(opts.model.value)) as Ref<T>;

  const model = computed({
    get: () => opts.model.value,
    set: value => {
      opts.model.value = value;
    },
  });

  const additionalValidationCheck = opts.isValid ?? (() => Promise.resolve([]));

  const takeModelSnapshot =
    opts.takeModelSnapshot ??
    function (model: T) {
      return JSON.stringify(transformData(model));
    };

  const additionalFieldErrors = computed(() => {
    return fieldsWithErrors.value.filter(field => field.isCustomValidation);
  });

  const revalidate = async (): Promise<boolean> => {
    const { valid: isFormValid, errors } = form.value
      ? await form.value.validate()
      : { valid: false, errors: [] };

    const additionalValidationErrors = await additionalValidationCheck(model.value);
    const isValid = isFormValid && !additionalValidationErrors.length;

    valid.value = isFormValid;
    fieldsWithErrors.value = [
      ...errors.map(error => ({
        field: error.id,
        errorMessages: error.errorMessages ?? [],
      })),
      ...additionalValidationErrors
        .filter(error => error.errorMessages?.length)
        .map(error => ({
          field: error.field,
          errorMessages: error.errorMessages,
          isCustomValidation: true,
        })),
    ];

    return isValid;
  };

  const submit = async (): Promise<void> => {
    try {
      submitted.value = false;
      const isValid = await revalidate();
      valid.value = isValid;

      if (!isValid) {
        return;
      }

      submitting.value = true;

      await opts.submit(model.value);

      edited.value = false;
      modelSnapshot.value = takeModelSnapshot(model.value);
      submitted.value = true;
      initialModel.value = deepClone(model.value);
    } catch (error) {
      logger.error('Failed to submit form', error);
    } finally {
      submitting.value = false;
    }
  };

  onMounted(() => {
    modelSnapshot.value = takeModelSnapshot(model.value);
  });

  watch(
    model,
    newModelValue => {
      edited.value = takeModelSnapshot(newModelValue) !== modelSnapshot.value;
    },
    { deep: true },
  );

  watch(
    () => form.value?.errors,
    errors => {
      valid.value = form.value?.isValid ?? false;
      fieldsWithErrors.value =
        errors?.map(error => ({
          field: error.id,
          errorMessages: error.errorMessages ?? [],
        })) ?? [];
    },
    { deep: true },
  );

  return {
    form,
    valid,
    edited,
    submitting,
    submitted,
    fieldsWithErrors,
    model,
    revalidate,
    submit,
    additionalFieldErrors,
    initialModel,
  };
};
