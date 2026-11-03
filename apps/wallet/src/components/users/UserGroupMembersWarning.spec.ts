import { flushPromises } from '@vue/test-utils';
import { describe, expect, it, vi } from 'vitest';
import { icAgent } from '~/core/ic-agent.core';
import { ListUsersResult, User } from '~/generated/station/station.did';
import { serviceManager } from '~/plugins/services.plugin';
import { StationService } from '~/services/station.service';
import { mount } from '~/test.utils';
import { ExtractOk } from '~/types/helper.types';
import UserGroupMembersWarning from './UserGroupMembersWarning.vue';

vi.mock('~/services/station.service', () => {
  const mock: Partial<StationService> = {
    withStationId: vi.fn().mockReturnThis(),
    listUsers: vi.fn().mockReturnThis(),
  };

  return {
    StationService: vi.fn(() => mock),
  };
});

const mockedStationService = new StationService(icAgent.get());
serviceManager.services.station = mockedStationService;

const mockListUsers = (names: string[], total = names.length) =>
  vi.spyOn(mockedStationService, 'listUsers').mockResolvedValueOnce({
    users: names.map(name => ({ name }) as User),
    privileges: [],
    next_offset: [],
    total: BigInt(total),
  } as ExtractOk<ListUsersResult>);

const mountWarning = async () => {
  const wrapper = mount(UserGroupMembersWarning, { props: { userGroupId: '1' } });
  await flushPromises();

  return wrapper.find('[data-test-id="user-group-members-warning"]');
};

describe('UserGroupMembersWarning', () => {
  it('lists the members that will be affected', async () => {
    mockListUsers(['Alice', 'Bob']);

    const warning = await mountWarning();

    expect(warning.text()).toContain(
      'Removing this group will affect users Alice, Bob who are still members of it.',
    );
  });

  it('uses the singular form for a single member', async () => {
    mockListUsers(['Alice']);

    const warning = await mountWarning();

    expect(warning.text()).toContain(
      'Removing this group will affect user Alice who is still a member of it.',
    );
  });

  it('mentions how many members are not listed', async () => {
    mockListUsers(['Alice', 'Bob'], 5);

    const warning = await mountWarning();

    expect(warning.text()).toContain('users Alice, Bob and 3 more who are still members of it.');
  });

  it('shows a generic warning when no members are listed', async () => {
    mockListUsers([]);

    const warning = await mountWarning();

    expect(warning.text()).toContain(
      'Removing this group will also remove it from any users who are still members of it.',
    );
  });

  it('shows a generic warning when the members are not visible to the caller', async () => {
    mockListUsers([], 3);

    const warning = await mountWarning();

    expect(warning.text()).toContain(
      'Removing this group will also remove it from any users who are still members of it.',
    );
  });

  it('shows a generic warning while the members are loading', async () => {
    vi.spyOn(mockedStationService, 'listUsers').mockReturnValueOnce(new Promise(() => {}));

    const warning = await mountWarning();

    expect(warning.text()).toContain(
      'Removing this group will also remove it from any users who are still members of it.',
    );
  });

  it('falls back to a generic warning when the members cannot be loaded', async () => {
    vi.spyOn(mockedStationService, 'listUsers').mockRejectedValueOnce(new Error('Unauthorized'));

    const warning = await mountWarning();

    expect(warning.text()).toContain(
      'Removing this group will also remove it from any users who are still members of it.',
    );
  });
});
