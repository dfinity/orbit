export enum CanisterMonitorSetupStep {
  Fund = 1,
  Obtain = 2,
}

export enum MonitoringStrategyEnum {
  BelowThreshold = 'BelowThreshold',
  BelowEstimatedRuntime = 'BelowEstimatedRuntime',
  Always = 'Always',
}

export enum ObtainCyclesStrategyEnum {
  StationDefault = 'StationDefault',
  MintFromNativeToken = 'MintFromNativeToken',
}
