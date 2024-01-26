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
    alpha_warning: 'Learn more about this alpha version. Use with caution.',
    wallet_info_card_title: '{name} Info',
    wallet_info_card_edit_btn: 'Edit wallet name',
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
    user_status_active: 'Active',
    user_status_inactive: 'Inactive',
    add_new_principal: 'Add new principal',
    principal_already_added: 'Principal already added.',
    user_associate_principal_warning:
      'Use with caution. The principal will be able to login as the user and perform actions on their behalf.',
    export_csv: 'Export CSV',
    params_parse_error: 'Failed to parse parameters, please try again.',
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
    types: {
      addusergroup: {
        short_title: 'Add user group',
        title: 'Add user group request',
      },
      unknown: {
        short_title: 'Unknown',
        title: 'Unknown request',
      },
    },
  },
  login: {
    signin_slogan: 'Securely connect to manage your crypto assets',
    auth_failed: 'Authentication failed, please try again.',
  },
  not_found: {
    title: 'Whoops, 404',
    description: 'The page you were looking for does not exist.',
    btn_back: 'Go back to home',
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
    no_wallets: 'No wallets avalable.',
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
    view: 'View',
    requested: 'Requested',
    proposals: 'Proposals',
    withdraw_requests: 'Withdraw Requests',
    approved: 'Approved',
    reject: 'Reject',
    balance: 'Balance',
    address: 'Address',
    confirm: 'Confirm',
    id: 'ID',
    save: 'Save',
    see_all: 'See All',
    requests: 'Requests',
    cancel: 'Cancel',
    rejected: 'Rejected',
    edit: 'Edit',
    destination_address: 'Destination address',
    search: 'Search',
    filters: 'Filters',
    reset: 'Reset',
    statuses: 'Statuses',
    token: 'Token',
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
    policies: 'Policies',
    transfers: 'Transfers',
    withdrawals: 'Withdrawals',
    transactions: 'Transactions',
    address_book: 'Address Book',
    new_transfer: 'New Transfer',
    completed: 'completed',
    pending: 'pending',
    new_withdraw: 'New withdraw',
    settings: 'Settings',
    close: 'Close',
    general: 'General',
    add: 'Add',
    remove: 'Remove',
    failed: 'Failed',
    owners: 'Owners',
    name: 'Name',
    processing: 'Processing',
    cancelled: 'Cancelled',
    user_name: 'User Name',
    scheduled: 'Scheduled',
    wallet_name: 'Wallet Name',
    identity_name: 'Identity Name',
    canister_id: 'Canister ID',
    principal: 'Principal',
    status: 'Status',
    control_panel: 'Control panel',
    confirmed: 'Confirmed',
    unconfirmed: 'Unconfirmed',
    main: 'Main',
    user_group: 'User Group',
    user_groups: 'User Groups',
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
      validUuidV4: 'This field must be a valid UUID v4.',
      duplicate: 'This field must be unique.',
      validTokenAmount: 'This field must contain a valid amount for the selected asset.',
    },
  },
  account_page: {
    not_found_title: 'Account not found',
    not_found_description: 'The account you are looking for could not be loaded.',
    not_found_btn: 'Go back to accounts',
  },
  navigation: {
    overview: 'Overview',
    accounts_transfers: 'Accounts & Transfers',
    address_book: 'Address Book',
    users: 'Users',
    settings: 'Settings',
    user_groups_permissions: 'User Groups & Permissions',
    administration: 'Wallet Administration',
    add_another_wallet: 'Add another wallet',
    account_info_settings: 'Account Info & Settings',
    login: 'Login',
    logout: 'Logout',
    proposals: 'Requests',
  },
  pages: {
    overview: {
      title: '{name} - Overview',
    },
    user_settings: {
      title: 'Account Info & Settings',
      subtitle: 'Configure preferences and manage your user.',
    },
    administration: {
      title: 'Wallet Administration',
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
    },
  },
  session: {
    expired_dialog_title: 'Your session has expired',
    expired_dialog_content: 'You must reauthenticate to continue.',
    expired_dialog_btn: 'Reauthenticate',
  },
};
