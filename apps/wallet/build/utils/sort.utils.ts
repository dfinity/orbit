const parseSemver = (version: string) => {
  const mainParts = version.split('-');
  const mainVersionParts = (mainParts?.[0] ?? '').split('.').map(Number);
  const major = mainVersionParts?.[0] ?? 0;
  const minor = mainVersionParts?.[1] ?? 0;
  const patch = mainVersionParts?.[2] ?? 0;
  const preRelease = mainParts?.[1] ? mainParts[1].split('.') : [];
  const preReleaseType = preRelease?.[0] ?? '';
  const preReleaseCounter = preRelease?.[1] ? preRelease[1].match(/^\d+/)?.[0] : undefined;

  return {
    major,
    minor,
    patch,
    preRelease: { type: preReleaseType, counter: parseInt(preReleaseCounter ?? '0', 10) },
  };
};

export const compareSemanticVersions =
  (sort: 'newest' | 'oldest' = 'newest') =>
  (a: string, b: string) => {
    const versionA = parseSemver(sort === 'newest' ? b : a);
    const versionB = parseSemver(sort === 'newest' ? a : b);

    if (versionA.major !== versionB.major) return versionA.major - versionB.major;
    if (versionA.minor !== versionB.minor) return versionA.minor - versionB.minor;
    if (versionA.patch !== versionB.patch) return versionA.patch - versionB.patch;

    if (versionA.preRelease.type !== versionB.preRelease.type)
      return versionA.preRelease.type.localeCompare(versionB.preRelease.type);

    return versionA.preRelease.counter - versionB.preRelease.counter;
  };
