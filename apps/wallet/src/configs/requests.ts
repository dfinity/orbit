import { RequestSpecifierEnum } from '~/types/station.types';

export const disabledRequestOperations = [
  RequestSpecifierEnum.ChangeExternalCanister,
  RequestSpecifierEnum.CreateExternalCanister,
];
