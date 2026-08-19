import { StationContext } from './station.core';

export type Severity = 'blocker' | 'warning' | 'info';

export interface Finding {
  checkId: string;
  severity: Severity;
  message: string;
  location?: string;
  fix?: string;
}

export interface AuditReport {
  ctx: StationContext;
  findings: Finding[];
  confirmations: string[];
}

const RANK: Record<Severity, number> = { blocker: 0, warning: 1, info: 2 };

const HEADER: Record<Severity, string> = {
  blocker: 'BLOCKERS',
  warning: 'WARNINGS',
  info: 'INFO',
};

const sortFindings = (findings: Finding[]): Finding[] =>
  [...findings].sort((a, b) => RANK[a.severity] - RANK[b.severity]);

const groupBySeverity = (findings: Finding[]): Map<Severity, Finding[]> => {
  const map = new Map<Severity, Finding[]>();
  for (const finding of findings) {
    const bucket = map.get(finding.severity) ?? [];
    bucket.push(finding);
    map.set(finding.severity, bucket);
  }
  return map;
};

export const renderReport = (report: AuditReport): string => {
  const lines: string[] = [];
  lines.push('Orbit Station Audit Report');
  lines.push(`station: ${report.ctx.station}`);
  lines.push(`network: ${report.ctx.network}`);
  lines.push('');

  const sorted = sortFindings(report.findings);
  const grouped = groupBySeverity(sorted);
  for (const severity of ['blocker', 'warning', 'info'] as Severity[]) {
    const bucket = grouped.get(severity) ?? [];
    if (bucket.length === 0) continue;
    lines.push(`==== ${HEADER[severity]} (${bucket.length}) ====`);
    lines.push('');
    for (const finding of bucket) {
      lines.push(`[${finding.checkId}]`);
      if (finding.location) lines.push(`  ${finding.location}`);
      lines.push(`  ${finding.message}`);
      if (finding.fix) lines.push(`  fix: ${finding.fix}`);
      lines.push('');
    }
  }

  if (report.confirmations.length > 0) {
    lines.push('==== Positive confirmations ====');
    for (const c of report.confirmations) lines.push(`- ${c}`);
    lines.push('');
  }

  const counts = sorted.reduce(
    (acc, f) => {
      acc[f.severity]++;
      return acc;
    },
    { blocker: 0, warning: 0, info: 0 } as Record<Severity, number>,
  );
  lines.push(`summary: ${counts.blocker} blocker, ${counts.warning} warning, ${counts.info} info`);

  return lines.join('\n');
};

export const exitCodeFor = (report: AuditReport): number => {
  if (report.findings.some(f => f.severity === 'blocker')) return 2;
  if (report.findings.some(f => f.severity === 'warning')) return 1;
  return 0;
};
