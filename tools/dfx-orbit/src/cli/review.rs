use crate::DfxOrbit;
use candid::Principal;
use station_api::{
    CallExternalCanisterOperationDTO, CanisterInstallMode, ChangeExternalCanisterOperationDTO,
};
use std::fmt::Write;

// TODO: Factor this out into multiple files

impl DfxOrbit {
    pub(crate) fn display_change_canister_operation(
        &self,
        output: &mut String,
        op: &ChangeExternalCanisterOperationDTO,
    ) -> anyhow::Result<()> {
        writeln!(output, "=== Change External Canister ===")?;
        writeln!(
            output,
            "Target: {}",
            self.try_reverse_lookup(&op.canister_id)
        )?;

        let mode = match op.mode {
            CanisterInstallMode::Install => "Install",
            CanisterInstallMode::Reinstall => "Reinstall",
            CanisterInstallMode::Upgrade => "Upgrade",
        };
        writeln!(output, "Mode: {}", mode)?;

        writeln!(output, "Module checksum: {}", &op.module_checksum)?;
        if let Some(arg_checksum) = &op.arg_checksum {
            writeln!(output, "Argument checksum: {}", arg_checksum)?;
        }
        Ok(())
    }

    pub(crate) fn display_call_canister_operation(
        &self,
        output: &mut String,
        op: &CallExternalCanisterOperationDTO,
    ) -> anyhow::Result<()> {
        writeln!(output, "=== Call External Canister ===")?;
        writeln!(
            output,
            "Execution method: \"{}\" of {}",
            op.execution_method.method_name,
            self.try_reverse_lookup(&op.execution_method.canister_id)
        )?;
        if let Some(validation_method) = &op.validation_method {
            writeln!(
                output,
                "Validation method: \"{}\" of {}",
                validation_method.method_name,
                self.try_reverse_lookup(&validation_method.canister_id)
            )?
        }
        if let Some(checksum) = &op.arg_checksum {
            writeln!(output, "Argument checksum: {}", checksum)?
        }
        if let Some(args) = &op.arg_rendering {
            writeln!(output, "Argument: {}", args)?
        }
        if let Some(cycles) = &op.execution_method_cycles {
            writeln!(output, "Execution method cycles: {}", cycles)?
        }
        if let Some(reply) = &op.execution_method_reply {
            match candid_parser::IDLArgs::from_bytes(reply) {
                // TODO: Check if we can get the type information from somewhere to annotate this with types
                Ok(response) => writeln!(output, "Execution response: {}", response),
                Err(_) => writeln!(output, "FAILED TO PARSE EXECUTION RESPONSE"),
            }?;
        }

        Ok(())
    }

    fn try_reverse_lookup(&self, canister_id: &Principal) -> String {
        match self.canister_name(canister_id).ok() {
            Some(canister_name) => {
                format!("{} ({})", canister_name, canister_id)
            }
            None => format!("{}", canister_id),
        }
    }
}
