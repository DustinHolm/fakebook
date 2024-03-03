import { FormControl, Input, FormHelperText } from "@mui/joy";
import { SxProps } from "@mui/joy/styles/types";
import { memo } from "react";
import { UseFormRegisterReturn } from "react-hook-form";

type FormInputProps = {
  registerProps: UseFormRegisterReturn;
  placeholder?: string;
  disabled?: boolean;
  error?: string;
  sx?: SxProps;
};

function _FormInput(props: FormInputProps) {
  const hasError = props.error !== undefined;
  return (
    <FormControl error={hasError} sx={props.sx}>
      <Input
        placeholder={props.placeholder}
        disabled={props.disabled}
        slotProps={{ input: { ...props.registerProps } }}
      />

      {hasError && <FormHelperText>{props.error}</FormHelperText>}
    </FormControl>
  );
}

export const FormInput = memo(_FormInput);
