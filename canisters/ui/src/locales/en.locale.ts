export default {
  app: {
    title: '{app} Wallet',
    action_save_failed: 'Failed to save action, please try again.',
    action_save_success: 'Action successfully saved.',
    session_load_error: 'Failed to load your session, please try again.',
    user_id: 'User ID',
    wallets: 'Wallets',
    confirm: 'Confirm',
    copied_to_clipboard: 'Value copied to clipboard.',
    initial_account_name: 'Main',
    alpha_warning: 'This is an alpha version. Use with caution.',
    wallet_info_card_title: '{name} Info',
    wallet_info_card_edit_btn: 'Edit wallet',
    wallet_info_card_remove_btn: 'Remove wallet',
    wallet_info_card_remove_btn_confirm: 'Are you sure you want to remove this wallet?',
    manage_associated_wallet: 'Manage associated wallet',
    manage_associated_wallet_hint: 'These settings only apply to your user and not to the wallet.',
    user_activities_card_title: 'User Activities',
    wallet_upgrades_card_title: 'Wallet Upgrades',
    data_load_error: 'Failed to load data, please try again.',
    dialog_confirmation_title: 'Confirmation',
    dialog_confirmation_question: 'Are you sure you want to continue with this action?',
    request_failed_message: 'Request failed, please try again.',
    request_pending_message: 'Your request was created and is pending for approval.',
    request_adopted_message: 'This request has been accepted and is being processed.',
    request_rejected_message: 'This request has been rejected.',
    request_completed_message: 'This request has been completed.',
    user_status_active: 'Active',
    user_status_inactive: 'Inactive',
    add_new_principal: 'Add new principal',
    principal_already_added: 'Principal already added.',
    user_associate_principal_warning:
      'Use with caution. The principal will be able to login as the user and perform actions on their behalf.',
    export_csv: 'Export CSV',
    params_parse_error: 'Failed to parse parameters, please try again.',
    submit_upgrade: 'Submit upgrade',
    canister_upgrade_target: 'Upgrade Target',
    canister_wasm_module: 'Canister Wasm Module',
    canister_upgrade_args_input: 'Upgrade arguments (optional)',
    canister_upgrade_args_input_hint: 'Only hex encoded arguments are accepted',
    search_items: 'Search items',
    search_users: 'Search users',
    search_user_groups: 'Search user groups',
    search_accounts: 'Search accounts',
    destination_source: 'Destination / Source',
    amount_token: 'Amount, {token}',
    no_transfers: 'No transfer found.',
    account_dialog_edit_criteria_hint: '* Approval criteria for updating account configurations',
    account_dialog_transfers_criteria_hint: '* Approval criteria for transfers',
    address_book_entry: 'Address Book Entry',
    notifications_panel_title: 'Notifications',
    notifications_panel_no_results: "You're all caught up.",
    notifications_panel_read_all: 'Read all',
    btn_home_back: 'Back to home',
    no_download_available: 'No download available.',
    failed_to_download_item: 'Failed to download {item}, please try again.',
  },
  blockchains: {
    icp: {
      name: 'Internet Computer',
      standards: {
        native: 'Native',
      },
    },
    eth: {
      name: 'Ethereum',
      standards: {
        native: 'Native',
      },
    },
    btc: {
      name: 'Bitcoin',
      standards: {
        native: 'Native',
      },
    },
  },
  change_canister: {
    targets: {
      upgradewallet: 'Wallet',
      upgradeupgrader: 'Upgrader',
    },
  },
  proposals: {
    proposed_by: 'Proposed by {name}',
    proposer_id: 'Proposer ID: {id}',
    title_info_message: 'The title set by the proposer.',
    no_results_found: 'No requests found.',
    status: {
      created: 'Pending',
      cancelled: 'Cancelled',
      adopted: 'Adopted',
      rejected: 'Rejected',
      completed: 'Completed',
      failed: 'Failed',
      processing: 'Processing',
      scheduled: 'Scheduled',
      unknown: 'Unknown',
    },
    processing_started_at: 'Processing started at {dt}',
    processing_completed_at: 'Processing completed at {dt}',
    processing_scheduled_at: 'Processing scheduled at {dt}',
    no_cancelled_reason: 'No reason specified.',
    no_failed_reason: 'No reason specified.',
    domains: {
      all: 'All',
      accounts: 'Accounts',
      address_book: 'Address book',
      system: 'System',
      transfers: 'Transfers',
      users: 'Users',
    },
    headers: {
      id: 'ID',
      status: 'Status',
      status_reason: 'Status Reason',
      created: 'Created',
      expires: 'Expires',
      operation_type: 'Operation Type',
      proposer: 'Proposer',
      details: 'Details',
      account_id: 'Account ID',
      account_name: 'Account Name',
      token: 'Token',
      address: 'Address',
      user_id: 'User ID',
      user_name: 'User Name',
      group_id: 'Group ID',
      group_name: 'Group Name',
      address_book_id: 'Address Book ID',
      blockchain: 'Blockchain',
      address_owner: 'Address Owner',
      policy_id: 'Policy ID',
      change_target: 'Change Target',
      wasm_checksum: 'Wasm Checksum',
      from_account: 'From Account',
      to: 'To Address',
      amount: 'Amount',
      fee: 'Fee',
    },
    download: {
      user_group: 'User Groups',
      user: 'Users',
      account: 'Accounts',
      access_policy: 'Access Policies',
      proposal_policy: 'Proposal Policies',
      address_book_entry: 'Address Book',
      change_canister: 'Upgrades',
      transfer: 'Transfers',
    },
    types: {
      addusergroup: {
        title: 'Add user group',
        request_title: 'Add user group request',
      },
      addaccount: {
        title: 'Add account',
        request_title: 'Add account request',
      },
      adduser: {
        title: 'Add user',
        request_title: 'Add user request',
      },
      addaccesspolicy: {
        title: 'Add access policy',
        request_title: 'Add access policy request',
      },
      addaddressbookentry: {
        title: 'Add address book entry',
        request_title: 'Add address book entry request',
      },
      addproposalpolicy: {
        title: 'Add approval policy',
        request_title: 'Add approval policy request',
      },
      removeproposalpolicy: {
        title: 'Remove approval policy',
        request_title: 'Remove approval policy request',
      },
      removeaccesspolicy: {
        title: 'Remove access policy',
        request_title: 'Remove access policy request',
      },
      removeusergroup: {
        title: 'Remove user group',
        request_title: 'Remove user group request',
      },
      removeaddressbookentry: {
        title: 'Remove address book entry',
        request_title: 'Remove address book entry request',
      },
      changecanister: {
        title: 'Change canister',
        request_title: 'Change canister request',
      },
      editaccesspolicy: {
        title: 'Edit access policy',
        request_title: 'Edit access policy request',
      },
      editusergroup: {
        title: 'Edit user group',
        request_title: 'Edit user group request',
      },
      edituser: {
        title: 'Edit user',
        request_title: 'Edit user request',
      },
      editaccount: {
        title: 'Edit account',
        request_title: 'Edit account request',
      },
      editaddressbookentry: {
        title: 'Edit address book entry',
        request_title: 'Edit address book entry request',
      },
      transfer: {
        title: 'Transfer',
        request_title: 'Transfer request',
      },
      editproposalpolicy: {
        title: 'Edit approval policy',
        request_title: 'Edit approval policy request',
      },
      unknown: {
        title: 'Unknown',
        request_title: 'Unknown request',
      },
    },
  },
  login: {
    signin_slogan: 'Securely connect to manage your crypto assets',
    auth_failed: 'Authentication failed, please try again.',
  },
  slogans: {
    elevate_to_orbit: {
      main: 'Elevate to {term1}, {term2}',
      term1: 'Orbit',
      term2: 'where security meets freedom',
    },
    institutions_multi_custody: {
      main: 'Where {term1} and {term2} align',
      term1: 'Institutions',
      term2: 'Multi-Custody',
    },
  },
  home: {
    welcome_back: 'Welcome back',
    notifications: {
      none: "You don't have new notifications.",
      some: 'You have {count} new notification(s).',
    },
  },
  footer: {
    copyright: '© 2023 - DFINITY Foundation',
    github: {
      description: 'Source Code',
    },
  },
  settings: {
    subtitle: 'Configure preferences and manage the associated identities of your user.',
    edit_success: 'Your user information has been successfully updated.',
    load_failed: 'Your user information failed to load, please try again.',
  },
  wallets: {
    add_account_proposal_saved: 'Account creation request sent',
    edit_account_proposal_saved: 'Account update request sent',
    pending_account_creation_subtitle: 'Pending account creation ...',
    proposal_failed_to_save: 'Proposal failed to save.',
    notification_failed_to_save: 'Notification failed to save.',
    no_accounts: 'No account available.',
    pending_proposals: 'Pending proposals',
    pending_requests: 'Pending requests',
    user_copied_to_clipboard: 'Wallet user copied.',
    account_address_copied_to_clipboard: 'Account address copied.',
    load_error: 'Failed to load wallets information, please try again.',
    load_error_withdraw_requests: 'Failed to load withdraw requests.',
    wallet_nr_title: '#{nr} Wallet',
    no_wallets: 'No wallets available.',
    user_load_error: 'Failed to load your wallet user.',
    no_wallet_user: 'No wallet user',
    please_register_to_continue: 'Please register with a wallet to continue',
    private_account: 'Private Account',
    joint_account: 'Joint Account',
    policy: 'Policy',
    policy_misconfigured: 'The account policies are misconfigured.',
    policy_config_unavailable: 'Policy configuration is unavailable.',
    policy_fixed_approval_threshold_desc: 'Exact number approvals required to operate the account',
    policy_variable_approval_threshold_desc:
      'Percentage of approvals required to operate the account',
    policies: {
      VariableApprovalThreshold: 'Percentage based approval',
      FixedApprovalThreshold: 'Exact number of approvals',
    },
    proposals: {
      transfer: {
        title: 'Approve transfer',
      },
    },
    no_deposit_found_search: 'No deposit found for the search criteria.',
    no_withdrawal_found_search: 'No withdrawal found for the search criteria.',
    no_withdraw_request_found_search: 'No withdraw request found for the search criteria.',
    add_wallet_list_item: 'Add existing wallet',
    add_wallet_dialog_title: 'Join existing wallet',
    add_wallet_dialog_already_added: 'This wallet is already added.',
  },
  terms: {
    deposits: 'Deposits',
    wallet: 'Wallet',
    all_done: 'All done',
    approve: 'Approve',
    create: 'Create',
    review: 'Review',
    type: 'Type',
    summary: 'Summary',
    metadata: 'Metadata',
    wasm: 'Wasm',
    arg: 'Arg',
    target: 'Target',
    download: 'Download',
    upgrader: 'Upgrader',
    view: 'View',
    new_address: 'New Address',
    requested: 'Requested',
    proposals: 'Proposals',
    specifier: 'Specifier',
    withdraw_requests: 'Withdraw Requests',
    approved: 'Approved',
    reject: 'Reject',
    balance: 'Balance',
    address: 'Address',
    min: 'Min',
    blockchain: 'Blockchain',
    address_owner: 'Address Owner',
    time: 'Time',
    criteria: 'Criteria',
    confirm: 'Confirm',
    id: 'ID',
    submit: 'Submit',
    none: 'None',
    save: 'Save',
    see_all: 'See All',
    requests: 'Requests',
    cancel: 'Cancel',
    checksum: 'Checksum',
    rejected: 'Rejected',
    edit: 'Edit',
    destination_address: 'Destination address',
    search: 'Search',
    filters: 'Filters',
    reset: 'Reset',
    statuses: 'Statuses',
    token: 'Token',
    configuration: 'Configuration',
    until: 'To',
    clear: 'Clear',
    to: 'To',
    from: 'From',
    account: 'Account',
    amount: 'Amount',
    send: 'Send',
    open: 'Open',
    created: 'Created',
    expires: 'Expires',
    created_at: 'Created at',
    expires_at: 'Expires at',
    yes: 'Yes',
    no: 'No',
    identities: 'Identities',
    asset: 'Asset',
    user: 'User',
    unknown: 'Unknown',
    user_id: 'User ID',
    login: 'Login',
    logout: 'Logout',
    signin: 'Sign In',
    signout: 'Sign Out',
    anonymous: 'Anonymous',
    new_account: 'Create Account',
    edit_account: 'Edit Account',
    accounts: 'Accounts',
    addresses: 'Addresses',
    policies: 'Policies',
    any: 'Any',
    transfers: 'Transfers',
    withdrawals: 'Withdrawals',
    transactions: 'Transactions',
    address_book: 'Address Book',
    resource: 'Resource',
    action: 'Action',
    new_transfer: 'New Transfer',
    approval_policy: 'Approval Policy',
    completed: 'completed',
    pending: 'pending',
    new_withdraw: 'New withdraw',
    settings: 'Settings',
    key: 'Key',
    value: 'Value',
    close: 'Close',
    general: 'General',
    add: 'Add',
    remove: 'Remove',
    failed: 'Failed',
    owners: 'Owners',
    name: 'Name',
    of: 'of',
    total: 'Total',
    processing: 'Processing',
    cancelled: 'Cancelled',
    user_name: 'User Name',
    scheduled: 'Scheduled',
    wallet_name: 'Wallet Name',
    users: 'Users',
    everyone: 'Everyone',
    identity_name: 'Identity Name',
    canister_id: 'Canister ID',
    principal: 'Principal',
    status: 'Status',
    transfer: 'Transfer',
    invalid: 'Invalid',
    control_panel: 'Control panel',
    confirmed: 'Confirmed',
    unconfirmed: 'Unconfirmed',
    main: 'Main',
    user_group: 'User Group',
    user_groups: 'User Groups',
    all: 'All',
    subset: 'Subset',
  },
  forms: {
    create: 'Create',
    edit: 'Edit',
    wallets: 'Wallets ({min}/{max})',
    identities: 'Identities ({min}/{max})',
    save_changes: 'Save Changes',
    rules: {
      required: 'This field is required.',
      maxLength: 'The maximum length of the {field} is {max} characters.',
      validPrincipal: 'This field must be a valid principal.',
      validCanisterId: 'This field must be a valid canister id.',
      validUuidV4: 'This field must be a valid UUID v4.',
      duplicate: 'This field must be unique.',
      validTokenAmount: 'This field must contain a valid amount for the selected asset.',
      requiredIntNumber: 'This field must be a valid integer number.',
      intNumberRange: '{field} must be between {min} and {max}.',
    },
  },
  navigation: {
    home: 'Home',
    accounts: 'Accounts',
    address_book: 'Address Book',
    users: 'Users',
    settings: 'Settings',
    user_groups_permissions: 'User Groups & Permissions',
    administration: 'Administration',
    add_another_wallet: 'Add another wallet',
    account_info_settings: 'Account Info & Settings',
    login: 'Login',
    logout: 'Logout',
    proposals: 'Requests',
    transfer_proposals: 'Transfer Requests',
    access_policies: 'Permissions',
    proposal_policies: 'Approval Policies',
  },
  pages: {
    accounts: {
      title: 'Accounts',
      btn_new_transfer: 'New transfer',
      btn_upload_csv: 'Upload CSV',
      error_fetching_account: 'Error fetching account, please try again.',
    },
    account: {
      not_found: 'Account not found',
      not_found_description: 'The account you are looking for does not exist.',
      csv_transfer_subtitle: 'Upload a CSV file to create multiple transfers at once.',
      csv_transfer_file_format_hint: 'The CSV file must contain the column "{to}" and "{amount}".',
      csv_transfer_file_column_to: 'to',
      csv_transfer_file_column_amount: 'amount',
      csv_transfer_file_rows_title: 'Transfers to be created: {count}',
      csv_ignored_transfers_hint: 'Transfers with errors will be ignored.',
      csv_transfer_failed: 'Failed to process transfers, please try again.',
    },
    address_book: {
      title: 'Address Book',
      btn_new_entry: 'New entry',
      no_results_found: 'No address book entry found.',
      error_fetching_address_book: 'Error fetching address book, please try again.',
    },
    user_settings: {
      title: 'Account Info & Settings',
      subtitle: 'Configure preferences and manage your user.',
    },
    administration: {
      title: 'Administration',
    },
    users: {
      title: 'Users',
      btn_new_user: 'Create user',
      create_new_user_title: 'Create new user',
      btn_edit_title: 'Edit user',
      error_fetching_users: 'Error fetching users, please try again.',
    },
    user_groups: {
      title: 'User Groups',
      btn_new_group: 'Create group',
      btn_manage_permissions: 'Manage Permissions',
      error_loading_user_groups: 'Error loading user groups, please try again.',
      btn_edit_title: 'Edit user group',
      create_new_group_title: 'Create new user group',
    },
    initialization: {
      status_starting: 'Initializing, please wait ...',
      status_deploying: 'Deploying your wallet to the Internet Computer ...',
      status_waiting_for_canister_initialization: 'Waiting for deployment to complete ...',
      status_creating_initial_account: 'Creating your initial account ...',
      status_completed: 'Your wallet has been successfully initialized, you`ll soon be redirected.',
      status_failed: 'Failed to initialize, please try again.',
    },
    proposals: {
      title: 'Requests',
      transfer_title: 'Transfer Requests',
    },
    access_policies: {
      title: 'Permissions',
      update_dialog_title: 'Update Permissions',
    },
    proposal_policies: {
      title: 'Approval Policies',
      create_label: 'Add Policy',
      dialog_title: 'Policy',
    },
    not_found: {
      title: 'Whoops, 404',
      subtitle: 'The page you were looking for does not exist.',
    },
    unauthorized: {
      title: 'Unauthorized',
      subtitle: 'You are not authorized to view this page.',
    },
    disconnected: {
      title: 'Disconnected',
      subtitle: 'You are not connected to the selected wallet.',
    },
    error: {
      title: 'Error',
      subtitle: 'An error occurred while loading the page.',
    },
  },
  session: {
    expired_dialog_title: 'Your session has expired',
    expired_dialog_content: 'You must reauthenticate to continue.',
    expired_dialog_btn: 'Reauthenticate',
  },
  access_policies: {
    resource_title: 'Resource',
    group_members_title: 'Members of groups',
    specific_users_title: 'Specific users',
    everyone_title: 'Everyone',
    individual_resources_title: 'Individual resource access',
    select_resource: 'Resource Type',
    resources: {
      account: 'Account',
      user: 'User',
      usergroup: 'User Group',
      accesspolicy: 'Access Policy',
      proposalpolicy: 'Proposal Policy',
      canistersettings: 'Canister Settings',
      changecanister: 'Change Canister',
      transfer: 'Transfer',
      proposal: 'Request',
      addressbook: 'Address Book',
    },
    actions: {
      list: 'List',
      create: 'Create',
      read: 'Read',
      update: 'Update',
      delete: 'Delete',
      readpublicconfig: 'Read public configuration',
      readsensitiveconfig: 'Read sensitive configuration',
    },
  },
  proposal_policies: {
    user_type_select: 'User type',
    add_criteria_label: 'Add criteria +',
    unsupported_specifier: 'Unsupported specifier definition',
    criteria_user_specifier: {
      owner: 'Owner',
      proposer: 'Proposer',
      any: 'Any user',
      group: 'Member of group',
      id: 'Specific user',
    },
    criteria: {
      and: 'all of',
      or: 'any of',
      not: 'none of',
      autoadopted: 'Auto-adopted',
      minimumvotes: 'Minimum votes',
      approvalthreshold: 'Approval threshold',
      hasaddressbookmetadata: 'Has address book metadata',
    },
    specifier: {
      editaccesspolicy: 'Edit permission',
      addusergroup: 'Add user group',
      removeproposalpolicy: 'Remove request policy',
      adduser: 'Add user',
      editusergroup: 'Edit user group',
      removeaddressbookentry: 'Remove address book entry',
      editaddressbookentry: 'Edit address book entry',
      addproposalpolicy: 'Add request policy',
      changecanister: 'Change canister',
      editproposalpolicy: 'Edit request policy',
      edituser: 'Edit user',
      transfer: 'Transfer',
      editaccount: 'Edit account',
      addaddressbookentry: 'Add address book entry',
      addaccesspolicy: 'Add permission',
      removeaccesspolicy: 'Remove permission',
      removeusergroup: 'Remove user group',
      addaccount: 'Add account',
    },
  },
};
