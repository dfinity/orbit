export default {
  app: {
    title: 'Portefeuille {app}',
    action_save_failed: "Échec de sauvegarde de l'action, veuillez essayer de nouveau.",
    action_save_success: "Sauvegarde de l'action réussie.",
    session_load_error: 'Échec du chargement votre session, veuillez essayer de nouveau.',
    user_id: 'Identifiant',
    wallets: 'Portefeuilles',
    confirm: 'Confirmez',
    copied_to_clipboard: 'Valeur placée dans le presse-papiers',
    initial_account_name: 'Principal',
    alpha_warning: 'Ceci est une version alpha, utilisez a vos risques',
    wallet_info_card_title: 'Info {name}',
    wallet_info_card_edit_btn: 'Éditer le portefeuille',
    wallet_info_card_remove_btn: 'Retirer le portefeuille',
    wallet_info_card_remove_btn_confirm: 'Êtes-vous sûrs de vouloir retirer ce portefeuille?',
    manage_associated_wallet: 'Gerer le portefeuille associé',
    manage_associated_wallet_hint:
      "Ces paramètres s'appliquent à votre utilisateur uniquement et non pas au portefeuille.",
    user_activities_card_title: "Activitiés de l'utilisateur",
    wallet_upgrades_card_title: 'Mise à jour du portefeuille',
    data_load_error: 'Échec de load data, veuillez essayer de nouveau.',
    dialog_confirmation_title: 'Confirmation',
    dialog_confirmation_question: 'Êtes vous certains de vouloir continuer cette action?',
    request_failed_message: 'La requête a echoué, veuillez essayer de nouveau.',
    request_pending_message: "Votre requête a été crée et est en attente d'approbation.",
    request_adopted_message: "Cette requête à été acceptée est en cours d'opération.",
    request_rejected_message: 'Cette requête à été rejectée.',
    request_completed_message: 'Cette requête à été completée.',
    user_status_active: 'Actif',
    user_status_inactive: 'Inactif',
    add_new_principal: 'Ajouter un nouveau principal',
    principal_already_added: 'Le principal a deja été ajouté.',
    user_associate_principal_warning:
      "Attention. Ce principal pourra acceder avec les permissions de l'utilisateur et prendre des actions en son nom.",
    export_csv: 'Exporter en CSV',
    params_parse_error: "Échec d'interpretation des paramètres, veuillez essayer de nouveau.",
    submit_upgrade: 'Soumettre une mise à jour',
    canister_upgrade_target: 'Cible de mise à jour',
    canister_wasm_module: 'Wasm Module du Canister',
    canister_upgrade_args_input: 'Paramètres de la mise à jour (optionnel)',
    canister_upgrade_args_input_hint: 'Seulement des paramètres encodés en hex sont acceptés',
    search_items: 'Rechercher des éléments',
    search_users: 'Rechercher des usagers',
    search_user_groups: "Rechercher des groupes d'usagers",
    search_accounts: 'Rechercher des comptes',
    destination_source: 'Destination / Source',
    amount_token: 'Montant, {token}',
    no_transfers: 'Aucun transfert trouvé.',
    account_dialog_edit_criteria_hint:
      "* Critères d'aprobation pour la mise à jour de la configuration du compte",
    account_dialog_transfers_criteria_hint: "* Critères d'aprobation des transferts",
    address_book_entry: "Entrée de carnet d'adresses",
    notifications_panel_title: 'Notifications',
    notifications_panel_no_results: 'Vous êtes à jour.',
    notifications_panel_read_all: 'Tout lire',
    btn_home_back: "Page d'acceuil",
    no_download_available: 'Pas de téléchargement disponible',
    failed_to_download_item: 'Échec de téléchargement {item}, veuillez essayer de nouveau.',
    download_error: 'Échec de téléchargement du fichier, veuillez essayer de nouveau.',
    leave_page_warning:
      'Êtes vous sûrs de vouloir quitter? Certains changements pourraient être perdus.',
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
    proposed_by: 'Proposé par {name}',
    proposer_id: 'Identifiant du proposeur: {id}',
    title_info_message: 'Titre choisis par le proposeur.',
    no_results_found: 'Aucun resultat trouvé.',
    status: {
      created: 'En Attente',
      cancelled: 'Annulé',
      adopted: 'Adopté',
      rejected: 'Rejeté',
      completed: 'Completé',
      failed: 'Échoué',
      processing: 'En Traitement',
      scheduled: 'Prévu',
      unknown: 'État Inconnu',
    },
    processing_started_at: 'Traitement commencé at {dt}',
    processing_completed_at: 'Traitement completé at {dt}',
    processing_scheduled_at: 'Traitement prévu à {dt}',
    no_cancelled_reason: 'Pas de raison déclarée.',
    no_failed_reason: 'Pas de raison déclarée.',
    domains: {
      all: 'Tout',
      accounts: 'Comptes',
      address_book: "Carnet d'Adresses",
      system: 'Système',
      transfers: 'Transferts',
      users: 'Usagers',
    },
    headers: {
      id: 'ID',
      status: 'État',
      status_reason: 'Raison',
      created: 'Crée',
      expires: 'Expire',
      operation_type: "Type d'Opération",
      proposer: 'Proposeur',
      details: 'Details',
      account_id: 'ID du Compte',
      account_name: 'Nom du Compte',
      token: 'Token',
      address: 'Addresse',
      user_id: "ID d'Usager",
      user_name: "Nom d'Usager",
      group_id: 'ID du Groupe',
      group_name: 'Nom du Groupe',
      address_book_id: "ID de Carnet d'Adresses",
      blockchain: 'Blockchain',
      address_owner: "Propriétaire de l'Adresse",
      policy_id: 'ID de Police',
      change_target: 'Cible',
      wasm_checksum: 'Checksum du Wasm',
      from_account: 'Du Compte',
      to: "A l'Adresse",
      amount: 'Montant',
      fee: 'Frais',
    },
    download: {
      user_group: "Groupes d'Usagers",
      user: 'Usagers',
      account: 'Comptes',
      access_policy: "Police d'Accès",
      proposal_policy: 'Police de Propositions',
      address_book_entry: "Carnet d'Adresses",
      change_canister: 'Mise à Jour',
      transfer: 'Transferts',
    },
    types: {
      addusergroup: {
        title: "Ajouter un groupe d'usagers",
        request_title: "Demander l'ajout d'un groupe d'uagers",
      },
      addaccount: {
        title: 'Ajouter un account',
        request_title: "Demander l'ajout d'un compte",
      },
      adduser: {
        title: 'Ajouter un user',
        request_title: "Demander l'ajout d'un usager",
      },
      addaccesspolicy: {
        title: "Ajouter une police d'accés",
        request_title: "Demander l'ajout d'une police d'accés ",
      },
      addaddressbookentry: {
        title: "Ajouter une entrée de carnet d'adresses",
        request_title: "Demander l'ajout d'une entrée de carnet d'adresses",
      },
      addproposalpolicy: {
        title: "Ajouter une police d'aprobation",
        request_title: "Demander l'ajout d'une police d'aprobation",
      },
      removeproposalpolicy: {
        title: "Supprimer police d'aprobation",
        request_title: "Supprimer police d'aprobation request",
      },
      removeaccesspolicy: {
        title: "Supprimer une police d'accès",
        request_title: "Demande de supprimer une police d'accès",
      },
      removeusergroup: {
        title: "Supprimer un groupe d'usagers",
        request_title: "Demande de supprimer un groupe d'usagers",
      },
      removeaddressbookentry: {
        title: "Supprimer une entrée de carnet d'adresses",
        request_title: "Demande de supprimer une entrée de carnet d'adresses",
      },
      changecanister: {
        title: 'Modifier un canister',
        request_title: 'Demande de modifier un canister',
      },
      editaccesspolicy: {
        title: "Modifier une police d'accès",
        request_title: "Demande de modifier une police d'accès",
      },
      editusergroup: {
        title: "Modifier un groupe d'usagers",
        request_title: "Demande de modifier un groupe d'usagers request",
      },
      edituser: {
        title: 'Modifier un usager',
        request_title: 'Demande de modifier un usager',
      },
      editaccount: {
        title: 'Modifier de modifier un compte',
        request_title: 'Demande de modifier un compte',
      },
      editaddressbookentry: {
        title: "Modifier une entrée de carnet d'adresses",
        request_title: "Demande de modifier une entrée de carnet d'adresses",
      },
      transfer: {
        title: 'Transfert',
        request_title: 'Demande de Transfert',
      },
      editproposalpolicy: {
        title: "Modifier police d'aprobation",
        request_title: "Demande de modifier une police d'aprobation request",
      },
      unknown: {
        title: 'Inconnu',
        request_title: 'Demande Inconnue',
      },
    },
  },
  login: {
    signin_slogan: 'Connectez vous de manière sécurisée pour gérer vos actifs crypto',
    auth_failed: "L'authentication a échoué, veuillez essayer de nouveau.",
  },
  slogans: {
    elevate_to_orbit: {
      main: 'Elevez vous en {term1}, {term2}',
      term1: 'Orbit',
      term2: 'où la sécurity rejoint la liberté ',
    },
    institutions_multi_custody: {
      main: "Où les {term1} et la {term2} s'alignent",
      term1: 'Institutions',
      term2: 'Garde-Partagée',
    },
  },
  home: {
    welcome_back: 'Bienvenue',
    notifications: {
      none: "Vous n'avez pas de nouvelles notifications.",
      some: 'Vous avez {count} nouvelles notification(s).',
    },
  },
  footer: {
    copyright: '© 2024 - DFINITY Foundation',
    github: {
      description: 'Code Source',
    },
  },
  settings: {
    subtitle: "Configurez vos paramètres et gerez les associations d'identité de vos usagers",
    edit_success: "Votre information d'usager a été mise à jour.",
    load_failed: "Votre information d'usager n'a pas été chargée, veuillez essayer de nouveau.",
  },
  wallets: {
    add_account_proposal_saved: 'Demande de création de compte envoyée',
    edit_account_proposal_saved: 'Demande de création de compte envoyée',
    pending_account_creation_subtitle: 'En attente de la création du compte...',
    proposal_failed_to_save: "La proposition n'a pas été sauvegardée.",
    notification_failed_to_save: "La notification n'a pas été sauvegardée.",
    no_accounts: 'Pas de compte disponible',
    pending_proposals: 'Propositions en attente',
    pending_requests: 'Demandes en attente',
    user_copied_to_clipboard: 'Usager copié.',
    account_address_copied_to_clipboard: 'Adresse du compte copiée.',
    load_error: 'Le chargement des portefeuils a échoué, veuillez essayer de nouveau.',
    load_error_withdraw_requests: 'Le chargement des demandes de retrait a échoué',
    wallet_nr_title: '#{nr} Portefeuille',
    no_wallets: 'Pas de portefeuille disponible.',
    user_load_error: 'Le chargement de votre usager de portefeuil a échoué.',
    no_wallet_user: "Pas d'usager de portefeuille",
    please_register_to_continue: 'Enregistrez un portefeuille pour continuer',
    private_account: 'Compte Privé',
    joint_account: 'Compte joint',
    policy: 'Police',
    policy_misconfigured: 'Les polices du compte sont mal configurées.',
    policy_config_unavailable: "La configuration de police n'est pas disponible.",
    policy_fixed_approval_threshold_desc:
      "Un nombre exact d'aprobations est requis pour executer des opérations sur le compte",
    policy_variable_approval_threshold_desc:
      "Un pourcentage d'aprobations est requis pour executer des opérations sur le compte",
    policies: {
      VariableApprovalThreshold: "Pourcentage d'aprobations",
      FixedApprovalThreshold: "Nombre exact d'aprobations",
    },
    proposals: {
      transfer: {
        title: 'Aprobation de transfert',
      },
    },
    no_deposit_found_search: 'Pas de depot trouvé pour ce critère.',
    no_withdrawal_found_search: 'Pas de retrait trouvé pour ce critère.',
    no_withdraw_request_found_search: 'Pas de demande retrait trouvé pour ce critère.',
    add_wallet_list_item: 'Ajouter un portefeuille',
    add_wallet_dialog_title: 'Rejoinde un portefeuille',
    add_wallet_dialog_already_added: 'Ce portefeuille est déjà present.',
  },
  terms: {
    deposits: 'Deports',
    wallet: 'Portefeuille',
    all_done: 'Tout terminé',
    approve: 'Aprouver',
    create: 'Créer',
    review: 'Revoir',
    type: 'Type',
    summary: 'Sommaire',
    metadata: 'Metadata',
    wasm: 'Wasm',
    arg: 'Paramètre',
    target: 'Cible',
    download: 'Télécharger',
    upgrader: 'Metteur à jour',
    view: 'Voir',
    new_address: 'Nouvelle Adresse',
    requested: 'Demandé',
    proposals: 'Demande',
    specifier: 'Spécificateur',
    withdraw_requests: 'Demandes de retrait',
    approved: 'Approuvé',
    reject: 'Rejetté',
    balance: 'Solde',
    address: 'Adresse',
    min: 'Min',
    blockchain: 'Blockchain',
    address_owner: "Propriétaire de l'adresse",
    time: 'date',
    criteria: 'Critère',
    confirm: 'Confirmer',
    id: 'ID',
    submit: 'Soumettre',
    none: 'Aucun',
    save: 'Sauvegarder',
    see_all: 'Voir Tout',
    requests: 'Demandes',
    cancel: 'Annuler',
    checksum: 'Checksum',
    rejected: 'Rejetté',
    edit: 'Modifier',
    destination_address: 'Adresse de destination',
    search: 'Rechercher',
    filters: 'Filtres',
    reset: 'Reinitilizer',
    statuses: 'Statuts',
    token: 'Token',
    configuration: 'Configuration',
    until: "Jusqu'a",
    clear: 'Effacer',
    to: 'À',
    from: 'De',
    account: 'Compte',
    amount: 'Montant',
    send: 'Envoyer',
    open: 'Ouvrir',
    created: 'Crée',
    expires: 'Expire',
    created_at: 'Crée à',
    expires_at: 'Expire à',
    yes: 'Oui',
    no: 'Non',
    identities: 'Identitités',
    asset: 'Actif',
    user: 'Usager',
    unknown: 'Inconnu',
    user_id: 'Identifiant',
    login: 'Se connecter',
    logout: 'Déconnecter',
    signin: 'Se connecter',
    signout: 'Se déconnecter',
    anonymous: 'Anonyme',
    new_account: 'Créer un compte',
    edit_account: 'Modifier le compte',
    accounts: 'Comptes',
    addresses: 'Adresses',
    policies: 'Polices',
    any: 'Tout',
    transfers: 'Transferts',
    withdrawals: 'Retraits',
    transactions: 'Transactions',
    address_book: "Carnet d'Adresses",
    resource: 'Resource',
    action: 'Action',
    new_transfer: 'Nouveau Transfert',
    approval_policy: "Police d'Aprobation",
    completed: 'completé',
    pending: 'en attente',
    new_withdraw: 'Nouveau retrait',
    settings: 'Paramètres',
    key: 'Key',
    value: 'Valeur',
    close: 'Fermer',
    general: 'Général',
    add: 'Ajouter',
    remove: 'Enlever',
    failed: 'Erreur',
    owners: 'Proprietaire',
    name: 'Nom',
    of: 'de',
    total: 'Total',
    processing: 'En Cours',
    cancelled: 'Annulé',
    user_name: "Nom d'utilisateur",
    scheduled: 'Planifié',
    wallet_name: 'Nom du Portefeuille',
    users: 'Usagers',
    everyone: 'Tout le monde',
    identity_name: "Nom de l'identité",
    canister_id: 'Canister ID',
    principal: 'Principal',
    status: 'Statut',
    transfer: 'Transfert',
    invalid: 'Invalide',
    control_panel: 'Paneau de Contrôle',
    confirmed: 'Confirmé',
    unconfirmed: 'non confirmé',
    main: 'Principal',
    user_group: "Groupe d'Usagers",
    user_groups: "Groupes d'Usagers",
    all: 'Tout',
    subset: 'sous-ensemble',
  },
  forms: {
    create: 'Créer',
    edit: 'Éditer',
    wallets: 'Portefeuilles ({min}/{max})',
    identities: 'Identités ({min}/{max})',
    save_changes: 'Sauvegarder',
    rules: {
      required: 'Ce champ est requis.',
      maxLength: 'La taille maximale du champ {field} est de {max} charactères.',
      validPrincipal: 'Ce champ doit contenir un principal valide',
      validCanisterId: 'Ce champ doit contenir un id de canister valide.',
      validUuidV4: 'Ce champ doit contenir un UUID v4 valide.',
      duplicate: 'Ce champ doit contenir un valeur unique.',
      validTokenAmount: "Ce champ doit contenir un montant valide pour l'actif sélectionné.",
      requiredIntNumber: 'Ce champ doit contenir un nombre entier valide.',
      intNumberRange: 'Le champ {field} doit contenir une valeur valide entre {min} et {max}.',
    },
  },
  navigation: {
    home: 'Acceuil',
    accounts: 'Comptes',
    address_book: "Carnet d'Adresses",
    users: 'Usagers',
    settings: 'Settings',
    user_groups_permissions: "Groupes d'Usagers et Accés",
    administration: 'Administration',
    add_another_wallet: 'Add another wallet',
    account_info_settings: 'Details du Compte et Paramètres',
    login: 'Se Connecter',
    logout: 'Deconnexion',
    proposals: 'Demandes',
    transfer_proposals: 'Demandes de Transfert',
    access_policies: "Polices d'Accés",
    proposal_policies: "Polices d'Aprobation",
  },
  pages: {
    accounts: {
      title: 'Comptes',
      btn_new_transfer: 'Nouveau Transfert',
      btn_upload_csv: 'Uploader un CSV',
      error_fetching_account: 'Erreur lors du chargement du compte, veuillez essayer de nouveau.',
    },
    account: {
      not_found: 'Pas de compte trouvé',
      not_found_description: "Le compte que vous cherchez n'a pas été trouvé.",
      csv_transfer_subtitle:
        "Uploader un fichier CSV pour créer plusieurs demandes de transfert d'un coup.",
      csv_transfer_file_format_hint:
        'Le fichier CSV doit contenir les colones "{to}" et "{amount}".',
      csv_transfer_file_column_to: 'de',
      csv_transfer_file_column_amount: 'montant',
      csv_transfer_file_rows_title: 'Transfers to be created: {count}',
      csv_ignored_transfers_hint: 'Transfers with errors will be ignored.',
      csv_transfer_failed: 'Échec de process transfers, veuillez essayer de nouveau.',
      csv_download_invalid: 'Téléchargement invalide',
    },
    address_book: {
      title: "Carnet d'Adresses",
      btn_new_entry: 'Nouvelle Adresse',
      no_results_found: "Aucune adresse trouvée dans le carnet d'adresses.",
      error_fetching_address_book:
        "Erreur de chargement du carnet d'adresses, veuillez essayer de nouveau.",
    },
    user_settings: {
      title: 'Détails du Compte et Paramètres',
      subtitle: 'Configurez vos paramètres et gerez votre usager.',
    },
    administration: {
      title: 'Administration',
    },
    users: {
      title: 'Usagers',
      btn_new_user: 'Créer un usager',
      create_new_user_title: 'Créer un nouvel usager',
      btn_edit_title: "Modifier l'usager",
      error_fetching_users: 'Erreur du chargement des usagers, veuillez essayer de nouveau.',
    },
    user_groups: {
      title: "Group d'Usagers",
      btn_new_group: "Ajouter un groupe d'usagers",
      btn_manage_permissions: "Gérer l'accès",
      error_loading_user_groups:
        "Erreur du chargement des groupes d'usagers, veuillez essayer de nouveau.",
      btn_edit_title: "Modifier le groupe d'usagers",
      create_new_group_title: "Ajouter un nouveau groupe d'usagers",
    },
    initialization: {
      status_starting: 'Initialization, veuillez patienter...',
      status_deploying: 'Deploiement de votre portefeuille au Internet Computer ...',
      status_waiting_for_canister_initialization: 'En attente de la fin du deploiement ...',
      status_creating_initial_account: 'Creation de votre compte initial ...',
      status_completed: "Your wallet has been successfully initialized, you'll soon be redirected.",
      status_failed: "Échec de l'initialization, veuillez essayer de nouveau.",
    },
    proposals: {
      title: 'Demandes',
      transfer_title: 'Demandes de Transfert',
    },
    access_policies: {
      title: 'Permissions',
      update_dialog_title: 'Modifier les Permissions',
    },
    proposal_policies: {
      title: "Polices d'Aprobation",
      create_label: 'Ajouter un police',
      dialog_title: 'Police',
    },
    not_found: {
      title: 'Oulala, 404',
      subtitle: "La page que vous cherchez n'existe pas.",
    },
    unauthorized: {
      title: 'Non Autorisé',
      subtitle: "Vous n'êtes pas autorisés à voir cette page.",
    },
    disconnected: {
      title: 'Déconnecté',
      subtitle: "Vous n'êtes pas connectés au portefeuille sélectionné.",
    },
    error: {
      title: 'Erreur',
      subtitle: 'Une erreur est survenue lors du chargement de la page.',
    },
  },
  session: {
    expired_dialog_title: 'Votre session a expiré',
    expired_dialog_content: 'Vous devez vous ré-authentifier pour continuer',
    expired_dialog_btn: 'Ré-authentifiez vous',
  },
  access_policies: {
    resource_title: 'Resource',
    group_members_title: 'Membres de groupes',
    specific_users_title: 'Usagers specifiques',
    everyone_title: 'Tout le monde',
    individual_resources_title: 'Accés à une resource individuelle',
    select_resource: 'Type de Resource',
    resources: {
      account: 'Compte',
      user: 'Usager',
      usergroup: "Groupe d'Usagers",
      accesspolicy: "Police d'Accés",
      proposalpolicy: 'Police de Demande',
      canistersettings: 'Paramètre de Canister',
      changecanister: 'Modification de Canister',
      transfer: 'Transfert',
      proposal: 'Demande',
      addressbook: "Carnet d'Adresses",
    },
    actions: {
      list: 'Lister',
      create: 'Créer',
      read: 'Lire',
      update: 'Modifier',
      delete: 'Éffacer',
      readpublicconfig: 'Lire les paramètres publiques',
      readsensitiveconfig: 'Lire les paramètres sensibles',
    },
  },
  proposal_policies: {
    user_type_select: "Type d'Usager",
    add_criteria_label: 'Ajouter un critère+',
    unsupported_specifier: 'Définition de spécificateur non supportée',
    criteria_user_specifier: {
      owner: 'Propriétaire',
      proposer: 'Demandeur',
      any: "N'importe quel usager",
      group: 'Membre du groupe',
      id: 'Usager spécifique',
    },
    criteria: {
      and: 'Tout les',
      or: 'Un des',
      not: 'Aucun des',
      autoadopted: 'Auto-adopté',
      minimumvotes: 'Votes minimum',
      approvalthreshold: "Seuil d'Aprobation",
      hasaddressbookmetadata: "Possède un attribut dans le carnet d'adresses",
    },
    specifier: {
      editaccesspolicy: 'Modifier les permissions',
      addusergroup: "Ajouter un groupe d'usagers",
      removeproposalpolicy: 'Éffacer une police de demande',
      adduser: 'Ajouter un usager',
      editusergroup: "Modifer le groupe d'usagers",
      removeaddressbookentry: "Éffacer une entrée dans le carnet d'adresses",
      editaddressbookentry: "Modifier une entrée dans le carnet d'adresses",
      addproposalpolicy: 'Ajouter une police de demande',
      changecanister: 'Modifier un canister',
      editproposalpolicy: 'Modifier une police de demande',
      edituser: 'Modifier un usager',
      transfer: 'Transfert',
      editaccount: 'Modifier un compte',
      addaddressbookentry: "Ajouter une entrée dans le carnet d'adresses",
      addaccesspolicy: 'Ajouter une permission',
      removeaccesspolicy: 'Éffacer une permission',
      removeusergroup: "Éffacer un groupe d'usagers",
      addaccount: 'Ajouter un compte',
    },
  },
};
