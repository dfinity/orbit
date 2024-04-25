import { ProposalCriteriaEnum, ProposalSpecifierEnum } from '~/types/wallet.types';

export const defaultCriterias = [
  ProposalCriteriaEnum.AutoAdopted,
  ProposalCriteriaEnum.ApprovalThreshold,
  ProposalCriteriaEnum.MinimumVotes,
  ProposalCriteriaEnum.And,
  ProposalCriteriaEnum.Or,
  ProposalCriteriaEnum.Not,
];

export const proposalSpecifiersIncludedCriterias = (): Record<
  ProposalSpecifierEnum,
  ProposalCriteriaEnum[]
> => ({
  [ProposalSpecifierEnum.Transfer]: [
    ProposalCriteriaEnum.HasAddressBookMetadata,
    ProposalCriteriaEnum.HasAddressInAddressBook,
    ...defaultCriterias,
  ],
  [ProposalSpecifierEnum.EditAccessPolicy]: [...defaultCriterias],
  [ProposalSpecifierEnum.AddProposalPolicy]: [...defaultCriterias],
  [ProposalSpecifierEnum.EditProposalPolicy]: [...defaultCriterias],
  [ProposalSpecifierEnum.RemoveProposalPolicy]: [...defaultCriterias],
  [ProposalSpecifierEnum.ChangeCanister]: [...defaultCriterias],
  [ProposalSpecifierEnum.AddUserGroup]: [...defaultCriterias],
  [ProposalSpecifierEnum.EditUserGroup]: [...defaultCriterias],
  [ProposalSpecifierEnum.RemoveUserGroup]: [...defaultCriterias],
  [ProposalSpecifierEnum.AddUser]: [...defaultCriterias],
  [ProposalSpecifierEnum.EditUser]: [...defaultCriterias],
  [ProposalSpecifierEnum.AddAccount]: [...defaultCriterias],
  [ProposalSpecifierEnum.EditAccount]: [...defaultCriterias],
  [ProposalSpecifierEnum.AddAddressBookEntry]: [...defaultCriterias],
  [ProposalSpecifierEnum.EditAddressBookEntry]: [...defaultCriterias],
  [ProposalSpecifierEnum.RemoveAddressBookEntry]: [...defaultCriterias],
});
