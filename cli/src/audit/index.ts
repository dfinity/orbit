import { createCommand } from 'commander';
import { writeFileSync } from 'fs';
import { resolve } from 'path';
import { assertReplicaIsHealthy } from '../utils';
import { assetEditPolicyWeakerThanTransfer } from './checks/asset-edit-policy-weaker-than-transfer';
import { externalCallValidationEqualsExecution } from './checks/external-call-validation-equals-execution';
import { quorumEmptyApproverSet } from './checks/quorum-empty-approver-set';
import {
  listAssets,
  listNamedRules,
  listPermissions,
  listRequestPolicies,
  listUserGroups,
  listUsers,
  StationContext,
} from './station.core';
import { AuditReport, exitCodeFor, Finding, renderReport } from './report';

const command = createCommand('audit').description(
  'Read-only sanity checks against an Orbit station configuration.',
);

command
  .requiredOption('-s, --station <CANISTER_ID>', 'The station canister id to audit.')
  .option('-n, --network <TYPE>', 'The network the station lives on. Defaults to `ic`.', 'ic')
  .option(
    '-i, --identity <TYPE>',
    'The dfx identity to call the station with (needs read access to list_* methods). Defaults to `default`.',
    'default',
  )
  .option(
    '-o, --output <PATH>',
    'Write the report to a file instead of stdout. The exit code is still set based on findings.',
  );

command.action(async options => {
  const ctx: StationContext = {
    station: options.station,
    network: options.network,
    identity: options.identity,
  };

  await assertReplicaIsHealthy(ctx.network);

  const [policies, users, userGroups, assets, namedRules, permissions] = await Promise.all([
    listRequestPolicies(ctx),
    listUsers(ctx),
    listUserGroups(ctx),
    listAssets(ctx),
    listNamedRules(ctx),
    listPermissions(ctx),
  ]);

  const findings: Finding[] = [
    ...quorumEmptyApproverSet(policies, users, namedRules),
    ...externalCallValidationEqualsExecution(policies, permissions),
    ...assetEditPolicyWeakerThanTransfer(policies, users, namedRules),
  ];

  const confirmations: string[] = [
    `${policies.length} request policies loaded.`,
    `${users.length} users loaded.`,
    `${userGroups.length} user groups loaded.`,
    `${assets.length} assets loaded.`,
    `${namedRules.length} named rules loaded.`,
    `${permissions.length} permissions loaded.`,
  ];

  const report: AuditReport = { ctx, findings, confirmations };
  const rendered = renderReport(report);

  if (options.output) {
    const target = resolve(options.output);
    writeFileSync(target, rendered + '\n', 'utf8');
    // Progress on stderr so file-output mode still gives feedback without
    // polluting the report file or stdout pipes.
    console.error(`Wrote audit report to ${target}`);
  } else {
    console.log(rendered);
  }

  process.exit(exitCodeFor(report));
});

export default command;
