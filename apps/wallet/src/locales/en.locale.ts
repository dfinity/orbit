export default {
  app: {
    title: '{app} Wallet',
    name: 'Orbit Wallet',
    action_save_failed: 'Failed to save action, please try again.',
    action_save_success: 'Action successfully saved.',
    session_load_error: 'Failed to load your session, please try again.',
    test_environment_warning_banner: {
      main: 'WARNING: Test environment.',
      info: 'Unstable features and data.',
    },
    api_compatibility_error:
      'Failed to check the compatibility of your wallet, you may experience issues.',
    stations: 'Wallets',
    confirm: 'Confirm',
    copied_to_clipboard: 'Value copied to clipboard.',
    initial_account_name: 'Main',
    station_info_card_title: '{name} Info',
    station_info_card_edit_btn: 'Preferences',
    station_info_card_edit_hint: 'Edit your own preferences and settings for this wallet.',
    station_info_card_remove_btn: 'Remove wallet',
    station_info_card_remove_btn_confirm: 'Are you sure you want to remove this wallet?',
    disaster_recovery_card_title: 'Disaster Recovery',
    disaster_recovery_not_configured: 'Disaster recovery not configured.',
    disaster_recovery_dialog_title: 'Configure Disaster Recovery',
    manage_associated_station: 'Manage associated wallet',
    manage_associated_station_hint: 'These settings only apply to your user and not to the wallet.',
    user_activities_card_title: 'User Activities',
    station_upgrades_card_title: 'Wallet Changes',
    data_load_error: 'Failed to load data, please try again.',
    dialog_confirmation_title: 'Confirmation',
    dialog_confirmation_question: 'Are you sure you want to continue with this action?',
    request_failed_message: 'Request failed, please try again.',
    request_pending_message: 'Your request was created and is pending for approval.',
    request_approved_message: 'This request has been approved and is being processed.',
    request_rejected_message: 'This request has been rejected.',
    request_completed_message: 'This request has been completed.',
    user_status_active: 'Active',
    user_status_inactive: 'Inactive',
    add_new_identity: 'Add new identity',
    principal_already_added: 'Principal already added.',
    user_associate_identity_warning:
      'Use with caution. The identity will be able to login as the user and perform actions on their behalf.',
    export_csv: 'Export CSV',
    params_parse_error: 'Failed to parse parameters, please try again.',
    software_update: 'Software Update',
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
    address_book_entry: 'Address Book Entry',
    notifications_panel_title: 'Notifications',
    notifications_panel_no_results: "You're all caught up.",
    notifications_panel_read_all: 'Read all',
    notifications_request_failed: 'Request failed: {reason}',
    btn_home_back: 'Back to home',
    no_download_available: 'No download available.',
    failed_to_download_item: 'Failed to download {item}, please try again.',
    download_error: 'Failed to download file, please try again.',
    leave_page_warning: 'Are you sure you want to leave? In progress changes will be lost.',
    loading_details: 'Loading details ...',
    account_dialog_create_new_title: 'Create new account',
    account_dialog_view_title: 'Account',
    account_dialog_access_read: 'Read',
    account_dialog_access_read_hint: 'Read only access to the account.',
    account_dialog_access_configuration: 'Change account settings',
    account_dialog_access_configuration_hint:
      'Access to change account settings, such as account name, policies, etc.',
    account_dialog_access_transfer: 'Transfer funds',
    account_dialog_access_transfer_hint: 'Access to transfer funds from the account.',
    account_dialog_request_policy_configuration: 'Change account settings',
    account_dialog_request_policy_configuration_hint:
      'The policy that needs to be approved to change account settings.',
    account_dialog_request_policy_transfer: 'Transfer funds',
    account_dialog_request_policy_transfer_hint:
      'The policy that needs to be approved to transfer funds.',
    request_policy_rule_builder_no_rule: 'No rule specified.',
    advanced_software_update_warning:
      'Use with caution. This is an advanced feature for updating the wallet.',
    check_updates_btn: 'Check for updates',
    update_recommended_latest:
      "It's recommended to keep your software up to date to ensure the best experience.",
    update_already_latest_version: 'You are already in the latest version.',
    checking_for_updates: 'Checking for updates ...',
    update_available: 'There is a new version available.',
    update_automated_comment: {
      summary: 'The {name} will be updated to version {version}.',
      verify_instructions:
        'To verify the update, open the terminal and follow the instructions bellow:',
    },
    no_data: 'No data available.',
    no_matching_results: 'No matching results found for `{search}`.',
    add_new_label: 'Add new label: {label}',
    user_cancel_pending_requests: 'Cancel all pending requests from this user.',
  },
  alpha_warning: {
    version: 'This is an alpha version.',
    caution: 'Use with caution.',
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
  system_upgrade: {
    targets: {
      upgradestation: 'Wallet',
      upgradeupgrader: 'Upgrader',
    },
  },
  requests: {
    unsupported_operation: 'Unsupported operation',
    requested_by: 'Requested by {name}',
    requester_id: 'Requester ID: {id}',
    title_info_message: 'The title set by the requester.',
    no_results_found: 'No requests found.',
    no_more_requests_to_approve: 'No more requests to approve.',
    load_next: 'Load next',
    status: {
      created: 'Pending',
      cancelled: 'Cancelled',
      approved: 'Approved',
      rejected: 'Rejected',
      completed: 'Completed',
      failed: 'Failed',
      processing: 'Processing',
      scheduled: 'Scheduled',
      unknown: 'Unknown',
    },
    approvals: 'Approvals',
    requester_auto_approval: 'Request automatically approved by the requester',
    approvals_and_evaluation: 'Approvals & rules',
    failure_title: 'Request execution failed',
    failure_reason_unknown: 'Request failed for an unspecified reason.',
    comment_optional: 'Comment (optional)',
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
      external_canisters: 'Canisters',
    },
    headers: {
      id: 'ID',
      status: 'Status',
      status_reason: 'Status Reason',
      created: 'Created',
      expires: 'Expires',
      operation_type: 'Operation Type',
      requester: 'Requester',
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
      from_account_address: 'From Address',
      to: 'To Address',
      amount: 'Amount',
      fee: 'Fee',
      comment: 'Comment',
    },
    download: {
      user_group: 'User Groups',
      user: 'Users',
      account: 'Accounts',
      permission: 'Access Policies',
      request_policy: 'Request Policies',
      address_book_entry: 'Address Book',
      system_upgrade: 'Upgrades',
      transfer: 'Transfers',
      external_canister: 'External Canisters',
      system_info: 'System Info',
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
      addaddressbookentry: {
        title: 'Add address book entry',
        request_title: 'Add address book entry request',
      },
      addrequestpolicy: {
        title: 'Add request policy',
        request_title: 'Add request policy request',
      },
      removerequestpolicy: {
        title: 'Remove request policy',
        request_title: 'Remove request policy request',
      },
      removeusergroup: {
        title: 'Remove user group',
        request_title: 'Remove user group request',
      },
      removeaddressbookentry: {
        title: 'Remove address book entry',
        request_title: 'Remove address book entry request',
      },
      systemupgrade: {
        title: 'System upgrade',
        request_title: 'System upgrade request',
      },
      editpermission: {
        title: 'Edit permission',
        request_title: 'Edit permission request',
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
      editrequestpolicy: {
        title: 'Edit request policy',
        request_title: 'Edit request policy request',
      },
      managesysteminfo: {
        title: 'Manage system info',
        request_title: 'Manage system info request',
      },
      createexternalcanister: {
        title: 'Create canister',
        request_title: 'Create canister request',
      },
      changeexternalcanister: {
        title: 'Change canister',
        request_title: 'Change canister request',
      },
      callexternalcanister: {
        title: 'Call canister',
        request_title: 'Call canister request',
      },
      fundexternalcanister: {
        title: 'Top-up canister',
        request_title: 'Top-up canister request',
      },
      configureexternalcanister: {
        title: 'Configure canister',
        request_title: 'Configure canister request',
      },
      setdisasterrecovery: {
        title: 'Edit disaster recovery',
        request_title: 'Edit disaster recovery request',
      },
      unknown: {
        title: 'Unknown',
        request_title: 'Unknown request',
      },
    },
    evaluation: {
      acceptance_rules: 'Acceptance rules',
      show_acceptance_rules: 'Show acceptance rules',
      hide_acceptance_rules: 'Hide acceptance rules',
      allof_rule: 'All of the following {n} rules:',
      anyof_rule: 'Any of the following {n} rules:',
      not_rule: 'Must not pass:',
      allowlisted_rule: 'Dest. address is in Address Book',
      not_found_in_allow_list: 'Not in Address Book',
      found_in_allow_list: 'In Address Book',
      allowlisted_with_metadata_rule: 'Dest. address has metadata in Address Book',
      allow_list_metadata_not_found: 'Not found {metadata}',
      allow_list_metadata_found: 'Found: {metadata}',
      quorum_rule: '1 minimum approving signature | {n} minimum approving signatures',
      quorum_percentage_rule: '1 minimum approving signature | {n} minimum approving signatures',
      approval_summary_approved: 'Approved with {n} for {m} against',
      approval_summary_rejected: 'Rejected with {n} for {m} against',
      approval_summary_pending: 'Pending with {n} for {m} against',
      approval_comments: '1 comment | {n} comments',
      auto_approved: 'Auto-approved',
      pending: 'Pending',
      rejected: 'Rejected',
      approved: 'Approved',
      summary_approved:
        'Request approved for the following reason: | Request approved for the following reasons:',
      summary_rejected:
        'Request rejected for the following reason: | Request rejected for the following reasons:',
      summary_pending:
        'Request pending for the following reason: | Request pending for the following reasons:',
      approved_reason_approval_quorum: 'user approval threshold met',
      approved_reason_allowlist: 'destination address was found in the address book',
      approved_reason_allowlist_metadata: 'destination address had metadata in the address book',
      reason_auto_approved: 'request was auto-approved',

      rejected_reason_approval_quorum: 'user approval threshold not met',
      rejected_reason_allowlist: 'destination address was not found in the address book',
      rejected_reason_allowlist_metadata:
        'destination address did not have metadata in the address book',

      pending_reason_approval_quorum: 'user approval pending',
      pending_reason_allowlist: 'destination address in the address book',
      pending_reason_allowlist_metadata: 'destination address in the address book with metadata',
    },
  },
  sidebar: {
    highlights: {
      main: 'Trustless Wallet {line1} {line2} {line3}',
      line3: 'Multichain',
      line1: 'Digital Assets',
      line2: 'Multi-Custody',
    },
  },
  landing: {
    title: 'Seamless Multichain',
    subtitle: 'One Platform, Full Control',
    description:
      'Orbit streamlines on-chain asset management for enterprises, DAOs, and teams, consolidating control and visibility into a single, intuitive platform.',
    connect_title: 'Securely connect to manage your digital assets',
    connect_btn: 'Connect with Internet Identity',
    connect_error: 'Failed to connect, please try again.',
  },
  home: {
    welcome_back: 'Welcome back',
    notifications: {
      none: "You don't have new notifications.",
      some: 'You have {count} new notification(s).',
    },
  },
  footer: {
    copyright: '© 2024 - DFINITY Foundation',
    github: {
      description: 'Source Code',
    },
  },
  settings: {
    subtitle: 'Configure preferences and manage the associated identities of your user.',
    edit_success: 'Your user information has been successfully updated.',
    load_failed: 'Your user information failed to load, please try again.',
  },
  stations: {
    add_account_request_saved: 'Account creation request sent',
    edit_account_request_saved: 'Account update request sent',
    pending_account_creation_subtitle: 'Pending account creation ...',
    request_failed_to_save: 'Request failed to save.',
    notification_failed_to_save: 'Notification failed to save.',
    no_accounts: 'No account available.',
    pending_requests: 'Pending requests',
    user_copied_to_clipboard: 'Wallet user copied.',
    account_address_copied_to_clipboard: 'Account address copied.',
    load_error: 'Failed to load wallets information, please try again.',
    load_error_withdraw_requests: 'Failed to load withdraw requests.',
    station_nr_title: '#{nr} Wallet',
    no_stations: 'No wallets available.',
    user_load_error: 'Failed to load your wallet user.',
    no_station_user: 'No wallet user',
    please_register_to_continue: 'Please register with a wallet to continue',
    private_account: 'Private Account',
    joint_account: 'Joint Account',
    policy: 'Policy',
    policy_misconfigured: 'The account policies are misconfigured.',
    policy_config_unavailable: 'Policy configuration is unavailable.',
    policy_fixed_approval_threshold_desc: 'Exact number approvals required to operate the account',
    policy_variable_approval_threshold_desc:
      'Percentage of approvals required to operate the account',
    requests: {
      transfer: {
        title: 'Approve transfer',
      },
    },
    no_deposit_found_search: 'No deposit found for the search criteria.',
    no_withdrawal_found_search: 'No withdrawal found for the search criteria.',
    no_withdraw_request_found_search: 'No withdraw request found for the search criteria.',
    add_station_list_item: 'Add wallet',
  },
  cycles: {
    units: {
      tc: 'TC',
      bc: 'Billion',
      mc: 'Million',
      e8s: 'e8s',
    },
  },
  external_canisters: {
    add_new_label: 'Add new label',
    use_existing: 'Use existing',
    create_new: 'Create new',
    initial_cycles: 'Initial Cycles',
    target_canister: 'Target Canister',
    config_read_permission: 'View',
    config_read_permission_hint: 'View only access to the canister.',
    config_change_permission: 'Changes',
    config_change_permission_hint: 'Access to change the canister settings.',
    config_change_approval_policy: 'Changes',
    config_change_approval_policy_hint: 'Approval policy to change the canister settings.',
    loading_error: 'Failed to load canister information, please try again.',
    not_found: 'Canister not found.',
    not_found_description: 'The canister you are looking for does not exist.',
    ic_settings: 'IC Settings',
    top_up: 'Top Up',
    configuration: 'Configuration',
    unlink: 'Unlink',
    unlink_title: 'Unlink Canister',
    unlink_soft_delete: 'Keep the canister on the Internet Computer, only remove its reference.',
    performed_calls: 'Performed calls',
    perform_call: {
      title: 'Perform Call',
      reply_received: 'Reply received',
      attached_cycles: 'Attached Cycles',
      argument: 'Argument',
      validated_argument: 'Validated argument',
      argument_checksum: 'Argument Checksum',
      method_name: 'Method Name',
      method_name_hint: 'The name of the method to call on the canister.',
      method_args: 'Arguments (optional)',
      method_args_hint: 'The arguments to pass to the method.',
      attach_cycles: 'Attach Cycles (optional)',
      attach_cycles_hint: 'The amount of cycles to attach to the call.',
      validation_method: 'Validation Method',
      validation_method_hint: 'The method to validate the call.',
      validation_method_item_remote: '{method} on canister "{canister}"',
      validation_method_item_none: 'No validation',
      call_submit_failed:
        'Call request failed, please make your arguments conform to the required validation.',
    },
    call_configuration: {
      title: 'Method call configurations',
      config_dialog_title: 'Method call configuration',
      add_new_method_pair: 'Add new method',
      edit_method_pair: 'Edit',
      no_configuration:
        'No method-specific configurations are defined for this canister, but global definitions might apply.',
      method_name: 'Method Name',
      method_name_hint: 'The name of the method to call on the canister or `*` for all methods.',
      method_call_permission: 'Permission',
      method_call_permission_hint:
        'The users who have permission to request the method call on the canister.',
      method_call_approval_policy: 'Approval Policy',
      method_call_approval_policy_hint:
        'The approval policy rules that needs to be approved to call the method on the canister.',
      advanced_validation: 'Payload validation (optional)',
      advanced_validation_hint:
        'Custom validation can enhance method security. They may throw an error if validation fails or return a formatted argument for reviewer context if successful.',
      add_advanced_validation: 'Add advanced validation',
      remove_advanced_validation: 'Remove advanced validation',
      validation_method_name: 'Validation Method Name',
      validation_method_name_hint:
        'The name of the validation method, must be different from the execution method.',
      validation_canister_id: 'Validation Canister ID',
      validation_canister_id_hint: 'The canister ID that contains the validation method.',
      card_validation_method_description: 'Validated with method {method} on canister {canister}',
      duplicated_configuration_error_type: 'Duplicated configuration',
      duplicated_method_call_configuration:
        'You already have a configuration for this method call, please edit the existing one.',
    },
    module_hash: 'Module Hash',
    cycles: 'Cycles',
    not_controller: 'Not controller',
    install: 'Install',
    send_cycles: 'Send Cycles',
    top_up_hint: 'Amount of cycles to add to the target canister.',
    add_controller: 'Add controller',
    no_controllers: 'No controllers',
    self_controller: 'Self controlled',
    non_orbit_controllers_warning:
      'This configuration sets additional controllers to the target canister, use with caution.',
    native_settings: {
      freezing_threshold: 'Freezing Threshold',
      freezing_threshold_hint:
        "The number of seconds the canister will be put in a frozen state before it's cleaned by the network.",
      controllers: 'Controllers',
      controllers_hint: 'The list of principals that can control the canister.',
      memory_allocation: 'Memory Allocation',
      memory_allocation_hint:
        'The maximum amount of memory the canister can use, this also reserves the memory in the subnet.',
      compute_allocation: 'Compute Allocation',
      compute_allocation_hint:
        'The percentage of subnet compute that is reserved for the canister.',
      reserved_cycles_limit: 'Reserved Cycles Limit',
      reserved_cycles_limit_hint:
        'Number of cycles the canister can allocate, operations that allocate memory or compute will fail if the limit is reached.',
    },
    wasm_module: 'WASM Module',
    wasm_args: 'Arguments',
    wasm_args_optional: 'Arguments (optional)',
    wasm_args_invalid_format: 'Invalid argument format',
    wasm_args_formats: {
      hex: 'Hex',
      candid: 'Candid',
    },
    install_mode: {
      reinstall: 'Reinstall',
      upgrade: 'Upgrade',
      install: 'Install',
    },
  },
  terms: {
    execute: 'Execute',
    error: 'Error',
    self: 'Self',
    more: 'More',
    less: 'Less',
    data: 'data',
    mode: 'Mode',
    active: 'Active',
    archived: 'Archived',
    canisters: 'Canisters',
    labels: 'Labels',
    canister: 'Canister',
    description: 'Description',
    change: 'Change',
    quorum: 'Quorum',
    deposits: 'Deposits',
    station: 'Wallet',
    all_done: 'All done',
    station_id: 'Wallet ID',
    details: 'Details',
    approve: 'Approve',
    create: 'Create',
    review: 'Review',
    identity: 'Identity',
    type: 'Type',
    summary: 'Summary',
    overriden: 'Overriden',
    metadata: 'Metadata',
    automated: 'Automated',
    advanced: 'Advanced',
    wasm: 'Wasm',
    arg: 'Arg',
    access: 'Access',
    previous: 'Previous',
    comment: 'Comment',
    comment_optional: 'Comment (optional)',
    next: 'Next',
    back: 'Back',
    permissions: 'Permissions',
    approval_policies: 'Approval Policies',
    target: 'Target',
    download: 'Download',
    upgrader: 'Upgrader',
    view: 'View',
    new_address: 'New Address',
    request: 'Request',
    requested: 'Requested',
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
    rule: 'Rule',
    confirm: 'Confirm',
    id: 'ID',
    submit: 'Submit',
    none: 'None',
    save: 'Save',
    see_all: 'See All',
    requests: 'Requests',
    cancel: 'Cancel',
    checksum: 'Checksum',
    module_checksum: 'Module Checksum',
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
    request_policy: 'Approval Policy',
    completed: 'completed',
    pending: 'pending',
    new_withdraw: 'New withdraw',
    settings: 'Settings',
    key: 'Key',
    value: 'Value',
    close: 'Close',
    general: 'General',
    update: 'Update',
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
    station_name: 'Wallet Name',
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
    user_group_id: 'User Group ID',
    user_groups: 'User Groups',
    all: 'All',
    subset: 'Subset',
    skip: 'Skip',
    version: 'Version',
    continue: 'Continue',
    cycle_obtain_strategy: 'Wallet top-up method',
  },
  forms: {
    create: 'Create',
    edit: 'Edit',
    stations: 'Wallets ({min}/{max})',
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
      validEmail: 'This field must be a valid email address.',
      requiredNumber: 'This field must be a valid number.',
      numberRange: 'This field must be between {min} and {max}.',
      invalidDecimalPlaces: 'This field must have a maximum of {decimals} decimal places.',
      isHex: 'This field must be a valid hexadecimal value.',
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
    add_another_station: 'Add another wallet',
    account_info_settings: 'Account Info & Settings',
    login: 'Login',
    logout: 'Logout',
    requests: 'Requests',
    transfer_requests: 'Transfer Requests',
    permissions: 'Permissions',
    request_policies: 'Request Policies',
    external_canisters: 'Canisters',
  },
  pages: {
    accounts: {
      title: 'Accounts',
      btn_new_transfer: 'New transfer',
      btn_upload_csv: 'Upload CSV',
      error_fetching_account: 'Error fetching account, please try again.',
      cycle_obtain_account: 'This account is used to top up the Orbit station cycle balance.',
    },
    account: {
      not_found: 'Account not found',
      not_found_description: 'The account you are looking for does not exist.',
      csv_transfer_subtitle: 'Upload a CSV file to create multiple transfers at once.',
      csv_transfer_file_format_hint:
        'The CSV file must contain the column "{to}" and "{amount}", and optionally "{comment}".',
      csv_transfer_file_column_to: 'to',
      csv_transfer_file_column_comment: 'comment',
      csv_transfer_file_column_amount: 'amount',
      csv_transfer_file_rows_title: 'Transfers to be created: {count}',
      csv_ignored_transfers_hint: 'Transfers with errors will be ignored.',
      csv_transfer_failed: 'Failed to process transfers, please try again.',
      csv_download_invalid: 'Download invalid',
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
      cycle_obtain_strategy_disabled:
        'WARNING: Station cycle balance top-up disabled. Your station may run out of cycles.',
      cycle_obtain_strategy_mint_from_native_token: 'Mint from ICP account',
    },
    users: {
      title: 'Users',
      btn_new_user: 'Create user',
      create_new_user_title: 'Create new user',
      btn_edit_title: 'Edit user',
      error_fetching_users: 'Error fetching users, please try again.',
    },
    external_canisters: {
      title: 'Canisters',
      btn_add_canister: 'Add canister',
      add_new_canister_title: 'Add new canister',
      edit_canister_title: 'Canister Configuration',
      error_fetching_canisters: 'Error fetching canisters, please try again.',
    },
    user_groups: {
      title: 'User Groups',
      btn_new_group: 'Create group',
      btn_manage_permissions: 'Manage Permissions',
      error_loading_user_groups: 'Error loading user groups, please try again.',
      btn_edit_title: 'Edit user group',
      create_new_group_title: 'Create new user group',
      disaster_recovery_group_tooltip: 'Members of this group can perform disaster recovery.',
    },
    add_station: {
      initialization_title: 'Welcome! How would you like to join Orbit?',
      add_station_title: 'How would you like to add a wallet?',
      option_join_existing_station: 'Join existing wallet',
      option_deploy_new_station: 'Create your own wallet',
      join_station_title: 'Join existing wallet',
      join_station_body:
        'Contact the owner to get the Wallet ID and send them your identity so that a user can be created for you.',
      join_station_canister_id: 'Wallet ID',
      join_station_name: 'Wallet Name',
      join_station: 'Join wallet',

      station_title: 'Create your own wallet',
      station_body:
        'Create your own wallet and manage your digital assets. You can add users, set permissions and manage request approval policies.',
      station_name_field: 'Wallet Name',
      admin_name_field: 'Your username',

      check_permissions_title: 'Checking waiting list status...',
      join_waitlist_title: 'Join waiting list',
      join_waitlist_body:
        "Join Orbit's waiting list! Enter your email to get early access and exclusive updates. Your journey starts now.",
      join_waitlist_email_field: 'Enter your email address',
      join_waitlist: 'Sign up now',

      waitlist_pending_title: 'You are on the waiting list!',
      waitlist_pending_body:
        'Please wait for the approval. You will receive an email once your request is approved.',

      waitlist_check_error_title: 'Failed to check waiting list status',
      waitlist_check_error_body: 'Failed to check waiting list status, please try again.',

      quota_exceed_error_title: 'Quota exceeded',
      quota_exceed_error_body: 'You have reached the maximum number of wallets you can create.',

      waitlist_denied_title: "You've been denied access.",
      waitlist_denied_body: 'Unfortunately, you are not eligible to join the waiting list.',

      status_starting: 'Initializing, please wait ...',
      status_deploying: 'Deploying your wallet to the Internet Computer ...',
      status_waiting_for_canister_initialization: 'Waiting for deployment to complete ...',
      status_creating_initial_account: 'Creating your initial account ...',
      status_completed: "Your wallet has been successfully initialized, you'll soon be redirected.",
      status_failed: 'Failed to initialize, please try again.',
    },
    requests: {
      title: 'Requests',
      transfer_title: 'Transfer Requests',
    },
    permissions: {
      title: 'Permissions',
      update_dialog_title: 'Update Permissions',
    },
    request_policies: {
      title: 'Request Policies',
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
      title_not_found_user_identity: 'You are not added to the wallet',
      subtitle_not_found_user_identity:
        'Contact the wallet owner to add a user for you with your principal.',

      title_other_station_error: 'Cannot connect to the wallet',
      subtitle_other_station_error: 'The wallet returned the following error:',

      title_canister_error: 'Cannot connect to the wallet',
      subtitle_canister_error:
        'There was an error accessing the wallet. Check your internet connection and that the wallet ID corresponds to a valid wallet.',
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
  permissions: {
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
      permission: 'Access Policy',
      requestpolicy: 'Request Policy',
      system: 'System',
      transfer: 'Transfer',
      request: 'Request',
      addressbook: 'Address Book',
      managesysteminfo: 'Manage System Info',
      externalcanister: 'External Canister',
    },
    actions: {
      list: 'List',
      create: 'Create',
      read: 'Read',
      update: 'Update',
      delete: 'Delete',
      transfer: 'Transfer',
      capabilities: 'Capabilities',
      systeminfo: 'System info',
      systeminfocapabilities: 'Capabilities (Supported Assets)',
      systeminfoconfig: 'Configuration (Upgrades, Metrics, Usage)',
      managesysteminfo: 'Manage System Info (e.g. name)',
      systemupgrade: 'Upgrade',
      change: 'Change',
      fund: 'Fund',
      callcanister: 'Call',
    },
    allow: {
      public: 'Anyone',
      authenticated: 'Authenticated',
      restricted: 'Restricted',
    },
  },
  request_policies: {
    user_type_select: 'User type',
    add_rule_label: 'Add rule +',
    unsupported_specifier: 'Unsupported specifier definition',
    rule_user_specifier: {
      owner: 'Owner',
      requester: 'Requester',
      any: 'Any user',
      group: 'Member of group',
      id: 'Specific user',
    },
    rule: {
      allof: 'all of',
      anyof: 'any of',
      not: 'none of',
      autoapproved: 'Auto-approved',
      quorum: 'Quorum',
      quorumpercentage: 'Quorum percentage',
      allowlistedbymetadata: 'Allowlisted by metadata',
      allowlisted: 'Allowlisted',
    },
    specifier: {
      editpermission: 'Edit permission',
      addusergroup: 'Add user group',
      removerequestpolicy: 'Remove request policy',
      adduser: 'Add user',
      editusergroup: 'Edit user group',
      removeaddressbookentry: 'Remove address book entry',
      editaddressbookentry: 'Edit address book entry',
      addrequestpolicy: 'Add request policy',
      systemupgrade: 'System upgrade',
      editrequestpolicy: 'Edit request policy',
      edituser: 'Edit user',
      transfer: 'Transfer',
      editaccount: 'Edit account',
      addaddressbookentry: 'Add address book entry',
      removeusergroup: 'Remove user group',
      addaccount: 'Add account',
      managesysteminfo: 'Manage system info',
      changeexternalcanister: 'Change canister',
      fundexternalcanister: 'Fund canister',
      setdisasterrecovery: 'Configure disaster recovery',
      callexternalcanister: 'Call canister',
      createexternalcanister: 'Create canister',
    },
  },
  cycle_obtain_strategies: {
    disabled: 'Disabled',
    mintfromnativetoken: 'Mint from ICP account',
  },
};
