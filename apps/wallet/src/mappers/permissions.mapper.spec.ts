import { describe, expect, it } from 'vitest';
import { fromResourceToDisplayText } from '~/mappers/permissions.mapper';

describe('PermissionsMapper', () => {
  it('should map resource to a display key', () => {
    expect(fromResourceToDisplayText({ Account: { List: null } })).toBe('account_list');
    expect(fromResourceToDisplayText({ Account: { Create: null } })).toBe('account_create');
    expect(fromResourceToDisplayText({ Account: { Read: { Id: '1' } } })).toBe('account_read_id');
    expect(fromResourceToDisplayText({ Account: { Read: { Any: null } } })).toBe(
      'account_read_any',
    );
    expect(fromResourceToDisplayText({ System: { ManageSystemInfo: null } })).toBe(
      'system_managesysteminfo',
    );
    expect(fromResourceToDisplayText({ System: { Upgrade: null } })).toBe('system_upgrade');
    expect(
      fromResourceToDisplayText({
        ExternalCanister: {
          Call: { execution_method: { Any: null }, validation_method: { No: null } },
        },
      }),
    ).toBe('externalcanister_call_execution_method_any_validation_method_no');
  });
});
