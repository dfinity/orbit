import { NxReleaseChangelogResult } from 'nx/src/command-line/release/changelog';
import { VersionData } from 'nx/src/command-line/release/version';

export interface ReleaseDetails {
  releaseId: number;
  versions: VersionData;
  changes: NxReleaseChangelogResult['projectChangelogs'];
}
