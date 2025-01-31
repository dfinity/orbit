import { computed, Ref, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { Allow, Resource } from '~/generated/station/station.did';
import { useStationStore } from '~/stores/station.store';
import { AggregatedResoucePermissions } from '~/types/permissions.types';

export const useResourcesFromAggregatedView = (
  aggregatedView: AggregatedResoucePermissions[],
): Resource[] => {
  const resources: Resource[] = [];

  for (const view of aggregatedView) {
    resources.push(...view.resources.map(r => r.resource));
  }

  return resources;
};

/**
 * Adds type safety to the permissions object to avoid defining permission keys multiple times.
 *
 * @param permissions The permissions to define
 * @returns Type safe permissions
 */
function definePermissions<
  T extends Record<
    string,
    {
      category: 'treasury' | 'canisters' | 'users' | 'system';
      resource: Resource;
      title: string;
      description: string;
    }
  >,
>(permissions: T): T {
  return permissions;
}

export const useGlobalPermissions = () => {
  const i18n = useI18n();

  return computed(() =>
    definePermissions({
      list_accounts: {
        category: 'treasury',
        resource: { Account: { List: null } },
        title: i18n.t('permissions.account_list'),
        description: i18n.t('permissions.account_list_description'),
      },
      create_account: {
        category: 'treasury',
        resource: { Account: { Create: null } },
        title: i18n.t('permissions.account_create'),
        description: i18n.t('permissions.account_create_description'),
      },
    }),
  );
};

// /**
//  * The categories are used to group permissions in the UI.
//  */
// export enum GlobalPermissionCategory {
//   Treasury = 'treasury',
//   Canisters = 'canisters',
//   Users = 'users',
//   System = 'system',
// }

// /**
//  * A single permission item that is displayed in the UI.
//  */
// export type GlobalPermissionItem = {
//   /**
//    * The type of the permission.
//    *
//    * - `view` permissions are used to display information in the UI, like a list of accounts.
//    * - `action` permissions are used to perform actions in the UI, like creating a new account.
//    */
//   type: 'view' | 'action';
//   /**
//    * The resource that this permission is related to.
//    */
//   resource: Resource;
//   /**
//    * The title of the permission.
//    *
//    * Example: "List accounts"
//    */
//   title: string;
//   /**
//    * The description of the permission.
//    *
//    * Example: "Allows the user to list all accounts"
//    */
//   description: string;
//   /**
//    * The current state of the permission.
//    *
//    * If `loading` is true, the permission is being fetched from the canister.
//    */
//   allow: { loading: true; error?: boolean } | { loading: false; value: Allow };
// };

// /**
//  * A map of global permissions that are displayed in the UI, grouped by category.
//  *
//  * The reason for using a two-dimensional array is to allow for grouping related permissions
//  * and separating them visually in the UI from the others of the same category.
//  */
// export type GlobalPermissions = { [key in GlobalPermissionCategory]: GlobalPermissionItem[][] };

// const getGlobalPermissions = (i18n: ReturnType<typeof useI18n>): GlobalPermissions => ({
//   treasury: [
//     [
//       {
//         type: 'view',
//         resource: { Account: { List: null } },
//         title: i18n.t('permissions.account_list.title'),
//         description: i18n.t('permissions.account_list.description'),
//         allow: { loading: true },
//       },
//       {
//         type: 'view',
//         resource: { Account: { Read: { Any: null } } },
//         title: i18n.t('permissions.account_read_any.title'),
//         description: i18n.t('permissions.account_read_any.description'),
//         allow: { loading: true },
//       },
//       {
//         type: 'action',
//         resource: { Account: { Create: null } },
//         title: i18n.t('permissions.account_create.title'),
//         description: i18n.t('permissions.account_create.description'),
//         allow: { loading: true },
//       },
//       {
//         type: 'action',
//         resource: { Account: { Update: { Any: null } } },
//         title: i18n.t('permissions.account_update_any.title'),
//         description: i18n.t('permissions.account_update_any.description'),
//         allow: { loading: true },
//       },
//       {
//         type: 'action',
//         resource: { Account: { Transfer: { Any: null } } },
//         title: i18n.t('permissions.account_transfer_any.title'),
//         description: i18n.t('permissions.account_transfer_any.description'),
//         allow: { loading: true },
//       },
//     ],
//     [
//       {
//         type: 'view',
//         resource: { AddressBook: { List: null } },
//         title: i18n.t('permissions.addressbook_list.title'),
//         description: i18n.t('permissions.addressbook_list.description'),
//         allow: { loading: true },
//       },
//       {
//         type: 'view',
//         resource: { AddressBook: { Read: { Any: null } } },
//         title: i18n.t('permissions.addressbook_read_any.title'),
//         description: i18n.t('permissions.addressbook_read_any.description'),
//         allow: { loading: true },
//       },
//       {
//         type: 'action',
//         resource: { AddressBook: { Create: null } },
//         title: i18n.t('permissions.addressbook_create.title'),
//         description: i18n.t('permissions.addressbook_create.description'),
//         allow: { loading: true },
//       },
//       {
//         type: 'action',
//         resource: { AddressBook: { Update: { Any: null } } },
//         title: i18n.t('permissions.addressbook_update_any.title'),
//         description: i18n.t('permissions.addressbook_update_any.description'),
//         allow: { loading: true },
//       },
//       {
//         type: 'action',
//         resource: { AddressBook: { Delete: { Any: null } } },
//         title: i18n.t('permissions.addressbook_delete_any.title'),
//         description: i18n.t('permissions.addressbook_delete_any.description'),
//         allow: { loading: true },
//       },
//     ],
//   ],
//   canisters: [],
//   users: [],
//   system: [],
// });

// export const useGlobalPermissions = (): {
//   permissions: Ref<GlobalPermissions>;
//   refresh: () => Promise<void>;
// } => {
//   const i18n = useI18n();
//   const station = useStationStore();
//   const permissions: Ref<GlobalPermissions> = ref(getGlobalPermissions(i18n));

//   const refresh = async (): Promise<void> => {
//     const resourcesToUpdate: Resource[] = [];
//     const resourcesPosition: {
//       // The key is a stringified version of the resource object to be able to fetch the position after the fetch.
//       [key: string]: {
//         category: GlobalPermissionCategory;
//         row: number;
//         col: number;
//       };
//     } = {};

//     for (const [category, permissionGroups] of Object.entries(permissions.value)) {
//       for (let row = 0; row < permissionGroups.length; row++) {
//         for (let col = 0; col < permissionGroups[row].length; col++) {
//           const permission = permissionGroups[row][col];
//           resourcesToUpdate.push(permission.resource);
//           resourcesPosition[JSON.stringify(permission.resource)] = {
//             category: category as GlobalPermissionCategory,
//             row,
//             col,
//           };
//         }
//       }
//     }

//     const fetchedPermissions = await station.service.listPermissions({
//       resources: [resourcesToUpdate],
//       paginate: [{ limit: [resourcesToUpdate.length], offset: [BigInt(0)] }],
//     });

//     const updatedPermissions = permissions.value;

//     for (const permission of fetchedPermissions.permissions) {
//       const position = resourcesPosition[JSON.stringify(permission.resource)];
//       updatedPermissions[position.category][position.row][position.col].allow = {
//         loading: false,
//         value: permission.allow,
//       };
//     }

//     // It is important to update the ref value to trigger a re-render.
//     permissions.value = updatedPermissions;
//   };

//   return { permissions, refresh };
// };
