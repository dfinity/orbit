export default {
  app: {
    title: 'Portefeuille {app}',
    action_save_failed: "Échec de sauvegarde de l'action, veuillez essayer de nouveau.",
    action_save_success: "Sauvegarde de l'action réussie.",
    session_load_error: 'Échec du chargement votre session, veuillez essayer de nouveau.',
    test_environment_warning_banner: {
      main: 'ATTENTION: Environnement de test.',
      info: 'Fonctionnalités et données instables.',
    },
    api_compatibility_error:
      'Échec de la vérification de la compatibilité de votre portefeuille, vous pouvez rencontrer des problèmes.',
    stations: 'Portefeuilles',
    confirm: 'Confirmez',
    copied_to_clipboard: 'Valeur placée dans le presse-papiers',
    initial_account_name: 'Principal',
    station_info_card_title: 'Info {name}',
    station_info_card_edit_btn: 'Préférences',
    station_info_card_edit_hint: 'Modifiez vos préférences et paramètres pour ce portefeuille.',
    station_info_card_remove_btn: 'Retirer le portefeuille',
    station_info_card_remove_btn_confirm: 'Êtes-vous sûrs de vouloir retirer ce portefeuille?',
    disaster_recovery_card_title: 'Sauvegarde',
    disaster_recovery_not_configured: 'Sauvegarde non configurée.',
    disaster_recovery_dialog_title: 'Configurer la sauvegarde',
    manage_associated_station: 'Gerer le portefeuille associé',
    manage_associated_station_hint:
      "Ces paramètres s'appliquent à votre utilisateur uniquement et non pas au portefeuille.",
    user_activities_card_title: "Activitiés de l'utilisateur",
    station_upgrades_card_title: 'Mise à jour du portefeuille',
    data_load_error: 'Échec de load data, veuillez essayer de nouveau.',
    dialog_confirmation_title: 'Confirmation',
    dialog_confirmation_question: 'Êtes vous certains de vouloir continuer cette action?',
    request_failed_message: 'La requête a echoué, veuillez essayer de nouveau.',
    request_pending_message: "Votre requête a été crée et est en attente d'approbation.",
    request_approved_message: "Cette requête à été approuvée est en cours d'opération",
    request_rejected_message: 'Cette requête à été rejectée.',
    request_completed_message: 'Cette requête à été completée.',
    user_status_active: 'Actif',
    user_status_inactive: 'Inactif',
    add_new_identity: 'Ajouter un nouveau identité',
    principal_already_added: 'Le principal a deja été ajouté.',
    user_associate_identity_warning:
      "Attention. Ce identité pourra acceder avec les permissions de l'utilisateur et prendre des actions en son nom.",
    export_csv: 'Exporter en CSV',
    params_parse_error: "Échec d'interpretation des paramètres, veuillez essayer de nouveau.",
    software_update: 'Mise à jour logicielle',
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
    address_book_entry: "Entrée de carnet d'adresses",
    notifications_panel_title: 'Notifications',
    notifications_panel_no_results: 'Vous êtes à jour.',
    notifications_panel_read_all: 'Tout lire',
    notifications_request_failed: 'Échec de la requête: {reason}',
    btn_home_back: "Page d'acceuil",
    no_download_available: 'Pas de téléchargement disponible',
    failed_to_download_item: 'Échec de téléchargement {item}, veuillez essayer de nouveau.',
    download_error: 'Échec de téléchargement du fichier, veuillez essayer de nouveau.',
    leave_page_warning:
      'Êtes vous sûrs de vouloir quitter? Certains changements pourraient être perdus.',
    loading_details: 'Chargement des détails...',
    account_dialog_create_new_title: 'Créer un nouveau compte',
    account_dialog_view_title: 'Compte',
    account_dialog_access_read: 'Lire',
    account_dialog_access_read_hint: 'Accès en lecture seule au compte.',
    account_dialog_access_configuration: 'Modifier les paramètres du compte',
    account_dialog_access_configuration_hint:
      'Accès pour modifier les paramètres du compte, tels que le nom du compte, les politiques, etc.',
    account_dialog_access_transfer: 'Transférer des fonds',
    account_dialog_access_transfer_hint: 'Accès pour transférer des fonds depuis le compte.',
    account_dialog_request_policy_configuration: 'Modifier les paramètres du compte',
    account_dialog_request_policy_configuration_hint:
      'La politique qui doit être approuvée pour modifier les paramètres du compte.',
    account_dialog_request_policy_transfer: 'Transférer des fonds',
    account_dialog_request_policy_transfer_hint:
      'La politique qui doit être approuvée pour transférer des fonds.',
    request_policy_rule_builder_no_rule: 'Pas de critères',
    advanced_software_update_warning:
      "À utiliser avec précaution. Il s'agit d'une fonctionnalité avancée pour mettre à jour le portefeuille.",
    check_updates_btn: 'Vérifier les mises à jour',
    update_recommended_latest:
      'Il est recommandé de garder votre logiciel à jour pour garantir la meilleure expérience.',
    update_already_latest_version: 'Vous êtes déjà à la dernière version.',
    checking_for_updates: 'Vérification des mises à jour ...',
    update_available: 'Une nouvelle version est disponible.',
    update_automated_comment: {
      summary: '{name} sera mis à jour vers la version {version}.',
      verify_instructions:
        'Pour vérifier la mise à jour, ouvrez le terminal et suivez les instructions ci-dessous:',
    },
  },
  alpha_warning: {
    version: 'Ceci est une version alpha.',
    caution: 'Utilisez avec prudence.',
  },
  sidebar: {
    highlights: {
      main: 'Portefeuille Trustless {line1} {line2} {line3}',
      line3: 'Multichain',
      line1: 'Actifs Numériques',
      line2: 'Multi-Garde',
    },
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
      upgradestation: 'Portefeuille',
      upgradeupgrader: 'Upgrader',
    },
  },
  requests: {
    unsupported_operation: 'Opération non supportée',
    requested_by: 'Demandé par {name}',
    requester_id: 'Identifiant du demand de {id}',
    title_info_message: 'Titre choisis par le demandeur,',
    no_results_found: 'Aucun resultat trouvé.',
    no_more_requests_to_approve: 'Pas de demandes à approuver.',
    load_next: 'Charger le suivant',
    status: {
      created: 'En Attente',
      cancelled: 'Annulé',
      approved: 'Approuvé',
      rejected: 'Rejeté',
      completed: 'Completé',
      failed: 'Échoué',
      processing: 'En Traitement',
      scheduled: 'Prévu',
      unknown: 'État Inconnu',
    },
    approvals: 'Approbations',
    requester_auto_approval: 'Demande approuvée automatiquement par le demandeur.',
    approvals_and_evaluation: 'Approbations et évaluation',
    failure_title: 'Échec de l execution de la demande',
    failure_reason_unknown: 'La demande',
    comment_optional: 'Commentaire (optionnel)',
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
      external_canisters: 'Canisters',
    },
    headers: {
      id: 'ID',
      status: 'État',
      status_reason: 'Raison',
      created: 'Crée',
      expires: 'Expire',
      operation_type: "Type d'Opération",
      requester: 'Demandeur',
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
      from_account_address: 'Adresse du Compte',
      to: "A l'Adresse",
      amount: 'Montant',
      fee: 'Frais',
      comment: 'Commentaire',
    },
    download: {
      user_group: "Groupes d'Usagers",
      user: 'Usagers',
      account: 'Comptes',
      permission: "Police d'Accès",
      request_policy: 'Police de Demande',
      address_book_entry: "Carnet d'Adresses",
      system_upgrade: 'Mise à Jour',
      transfer: 'Transferts',
      external_canister: 'Canister Géré',
      system_info: 'Informations Système',
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
      addaddressbookentry: {
        title: "Ajouter une entrée de carnet d'adresses",
        request_title: "Demander l'ajout d'une entrée de carnet d'adresses",
      },
      addrequestpolicy: {
        title: "Ajouter une police d'aprobation",
        request_title: "Demander l'ajout d'une police d'aprobation",
      },
      removerequestpolicy: {
        title: "Supprimer police d'aprobation",
        request_title: "Supprimer police d'aprobation request",
      },
      removeusergroup: {
        title: "Supprimer un groupe d'usagers",
        request_title: "Demande de supprimer un groupe d'usagers",
      },
      removeaddressbookentry: {
        title: "Supprimer une entrée de carnet d'adresses",
        request_title: "Demande de supprimer une entrée de carnet d'adresses",
      },
      systemupgrade: {
        title: 'Mise à jour du système',
        request_title: 'Demande de mise à jour du système',
      },
      editpermission: {
        title: 'Modifier les permissions',
        request_title: 'Demande de modifier une permission',
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
      editrequestpolicy: {
        title: "Modifier police d'aprobation",
        request_title: "Demande de modifier une police d'aprobation request",
      },
      managesysteminfo: {
        title: 'Gérer les informations système',
        request_title: 'Demande de gérer les informations système',
      },
      unknown: {
        title: 'Inconnu',
        request_title: 'Demande Inconnue',
      },
    },
    evaluation: {
      acceptance_rules: 'Règles d acceptation',
      show_acceptance_rules: 'Afficher les règles d acceptation',
      hide_acceptance_rules: 'Cacher les règles d acceptation',
      allof_rule: 'Toutes les règles suivantes {n}:',
      anyof_rule: 'Une des règles suivantes {n}:',
      not_rule: 'Ne doit pas passer:',
      allowlisted_rule: 'L adresse de destination est dans le carnet d adresses',
      not_found_in_allow_list: 'Pas dans le carnet d adresses',
      found_in_allow_list: 'Dans le carnet d adresses',
      allowlisted_with_metadata_rule:
        'L adresse de destination a des métadonnées dans le carnet d adresses',
      allow_list_metadata_not_found: 'Non trouvé {metadata}',
      allow_list_metadata_found: 'Trouvé: {metadata}',
      quorum_rule: '1 signature d approbation minimum | {n} signatures d approbation minimum',
      quorum_percentage_rule:
        '1 signature d approbation minimum | {n} signatures d approbation minimum',
      approval_summary_approved: 'Approuvé avec {n} pour {m} contre',
      approval_summary_rejected: 'Rejeté avec {n} pour {m} contre',
      approval_summary_pending: 'En attente avec {n} pour {m} contre',
      approval_comments: '1 commentaire | {n} commentaires',
      auto_approved: 'Auto-approuvé',
      pending: 'En attente',
      rejected: 'Rejeté',
      approved: 'Approuvé',

      summary_approved:
        'Demande approuvée pour la raison suivante: | Demande approuvée pour les raisons suivantes:',
      summary_rejected:
        'Demande rejetée pour la raison suivante: | Demande rejetée pour les raisons suivantes:',
      summary_pending:
        'Demande en attente pour la raison suivante: | Demande en attente pour les raisons suivantes:',
      approved_reason_approval_quorum: 'seuil d approbation utilisateur atteint',
      approved_reason_allowlist: 'adresse de destination trouvée dans le carnet d adresses',
      approved_reason_allowlist_metadata:
        'adresse de destination avait des métadonnées dans le carnet d adresses',
      reason_auto_approved: 'demande auto-approuvée',
      rejected_reason_approval_quorum: 'seuil d approbation utilisateur non atteint',
      rejected_reason_allowlist: 'adresse de destination non trouvée dans le carnet d adresses',
      rejected_reason_allowlist_metadata:
        'adresse de destination n avait pas de métadonnées dans le carnet d adresses',
      pending_reason_approval_quorum: 'approbation utilisateur en attente',
      pending_reason_allowlist: 'adresse de destination dans le carnet d adresses',
      pending_reason_allowlist_metadata:
        'adresse de destination dans le carnet d adresses avec des métadonnées',
    },
  },
  landing: {
    title: 'Gestion Multichain',
    subtitle: 'Une Plateforme, Une Supervision Complète',
    description:
      'Orbit simplifie la gestion des actifs on-chain pour les entreprises, les DAOs et les équipes, en consolidant le contrôle et la visibilité sur une seule plateforme intuitive.',
    connect_title: 'Connectez-vous en toute sécurité pour gérer vos actifs numériques',
    connect_btn: 'Se connecter avec Internet Identity',
    connect_error: 'Échec de la connexion, veuillez réessayer.',
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
  stations: {
    add_account_request_saved: 'Demande de création de compte envoyée',
    edit_account_request_saved: 'Demande de création de compte envoyée',
    pending_account_creation_subtitle: 'En attente de la création du compte...',
    request_failed_to_save: "La demande n'a pas été sauvegardée.",
    notification_failed_to_save: "La notification n'a pas été sauvegardée.",
    no_accounts: 'Pas de compte disponible',
    pending_requests: 'Demandes en attente',
    user_copied_to_clipboard: 'Usager copié.',
    account_address_copied_to_clipboard: 'Adresse du compte copiée.',
    load_error: 'Le chargement des portefeuils a échoué, veuillez essayer de nouveau.',
    load_error_withdraw_requests: 'Le chargement des demandes de retrait a échoué',
    station_nr_title: '#{nr} Portefeuille',
    no_stations: 'Pas de portefeuille disponible.',
    user_load_error: 'Le chargement de votre usager de portefeuil a échoué.',
    no_station_user: "Pas d'usager de portefeuille",
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
    requests: {
      transfer: {
        title: 'Aprobation de transfert',
      },
    },
    no_deposit_found_search: 'Pas de depot trouvé pour ce critère.',
    no_withdrawal_found_search: 'Pas de retrait trouvé pour ce critère.',
    no_withdraw_request_found_search: 'Pas de demande retrait trouvé pour ce critère.',
    add_station_list_item: 'Ajouter un portefeuille',
  },
  terms: {
    active: 'Actif',
    archived: 'Archivé',
    canisters: 'Canisters',
    canister: 'Canister',
    labels: 'Étiquettes',
    change: 'Changement',
    quorum: 'Quorum',
    deposits: 'Deports',
    station: 'Portefeuille',
    all_done: 'Tout terminé',
    approve: 'Aprouver',
    station_id: 'ID du Portefeuille',
    details: 'Détails',
    identity: 'Identité',
    create: 'Créer',
    review: 'Revoir',
    overriden: 'Remplacé',
    type: 'Type',
    summary: 'Sommaire',
    metadata: 'Metadata',
    wasm: 'Wasm',
    comment: 'Commentaire',
    comment_optional: 'Commentaire (optionnel)',
    arg: 'Paramètre',
    target: 'Cible',
    previous: 'Précédent',
    next: 'Suivant',
    automated: 'Automatisé',
    advanced: 'Avancé',
    back: 'Retour',
    permissions: 'Permissions',
    approval_policies: "Politiques d'approbation",
    download: 'Télécharger',
    upgrader: 'Metteur à jour',
    view: 'Voir',
    new_address: 'Nouvelle Adresse',
    request: 'Demande',
    requested: 'Demandé',
    requests: 'Demandes',
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
    rule: 'Règle',
    confirm: 'Confirmer',
    id: 'ID',
    submit: 'Soumettre',
    none: 'Aucun',
    save: 'Sauvegarder',
    see_all: 'Voir Tout',
    cancel: 'Annuler',
    checksum: 'Checksum',
    module_checksum: 'Checksum du Module',
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
    request_policy: "Police d'Aprobation",
    completed: 'completé',
    pending: 'en attente',
    new_withdraw: 'Nouveau retrait',
    settings: 'Paramètres',
    key: 'Key',
    value: 'Valeur',
    close: 'Fermer',
    general: 'Général',
    update: 'Mettre à jour',
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
    station_name: 'Nom du Portefeuille',
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
    user_group_id: 'ID du Groupe d Usagers',
    user_groups: "Groupes d'Usagers",
    all: 'Tout',
    subset: 'sous-ensemble',
    skip: 'Sauter',
    version: 'Version',
    continue: 'Continuer',
    cycle_obtain_strategy: 'Méthode de recharge du portefeuille',
  },
  forms: {
    create: 'Créer',
    edit: 'Éditer',
    stations: 'Portefeuilles ({min}/{max})',
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
      validEmail: 'Ce champ doit contenir une adresse email valide.',
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
    add_another_station: 'Ajouter un autre portefeuille',
    account_info_settings: 'Details du Compte et Paramètres',
    login: 'Se Connecter',
    logout: 'Deconnexion',
    requests: 'Demandes',
    transfer_requests: 'Demandes de Transfert',
    permissions: "Polices d'Accés",
    request_policies: "Polices d'Aprobation",
    external_canisters: 'Canisters',
  },
  pages: {
    accounts: {
      title: 'Comptes',
      btn_new_transfer: 'Nouveau Transfert',
      btn_upload_csv: 'Uploader un CSV',
      error_fetching_account: 'Erreur lors du chargement du compte, veuillez essayer de nouveau.',
      cycle_obtain_account:
        'Ce compte est utilisé pour recharger le solde de cycles de la station Orbit.',
    },
    account: {
      not_found: 'Pas de compte trouvé',
      not_found_description: "Le compte que vous cherchez n'a pas été trouvé.",
      csv_transfer_subtitle:
        "Uploader un fichier CSV pour créer plusieurs demandes de transfert d'un coup.",
      csv_transfer_file_format_hint:
        'Le fichier CSV doit contenir les colones "{to}" et "{amount}", et optionnellement "{comment}".',
      csv_transfer_file_column_to: 'de',
      csv_transfer_file_column_comment: 'commentaire',
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
      cycle_obtain_strategy_disabled: 'Stratégie de recharge des cycles non définie',
      cycle_obtain_strategy_mint_from_native_token: 'Mint depuis le compte ICP',
    },
    users: {
      title: 'Usagers',
      btn_new_user: 'Créer un usager',
      create_new_user_title: 'Créer un nouvel usager',
      btn_edit_title: "Modifier l'usager",
      error_fetching_users: 'Erreur du chargement des usagers, veuillez essayer de nouveau.',
    },
    external_canisters: {
      title: 'Canisters',
      btn_add_canister: 'Ajouter un canister',
      add_new_canister_title: 'Ajouter un nouveau canister',
      error_fetching_canisters: 'Erreur du chargement des canisters, veuillez essayer de nouveau.',
    },
    user_groups: {
      title: "Group d'Usagers",
      btn_new_group: "Ajouter un groupe d'usagers",
      btn_manage_permissions: "Gérer l'accès",
      error_loading_user_groups:
        "Erreur du chargement des groupes d'usagers, veuillez essayer de nouveau.",
      btn_edit_title: "Modifier le groupe d'usagers",
      create_new_group_title: "Ajouter un nouveau groupe d'usagers",
      disaster_recovery_group_tooltip:
        'Les membres de ce groupe peuvent effectuer une récupération après sinistre.',
    },
    add_station: {
      initialization_title: 'Bienvenue! Comment aimeriez-vous rejoindre Orbit?',
      add_station_title: 'Comment aimeriez-vous ajouter un portefeuille?',
      option_join_existing_station: 'Rejoindre un portefeuille existant',
      option_deploy_new_station: 'Déployer un nouveau portefeuille',
      check_permissions_title: 'Vérification du statut de la liste d attente...',
      join_waitlist_title: 'Rejoindre la liste d attente',
      join_waitlist_body:
        "Rejoignez la liste d'attente d'Orbit! Entrez votre email pour obtenir un accès anticipé et des mises à jour exclusives. Votre voyage commence maintenant.",
      join_waitlist_email_field: 'Entrez votre adresse e-mail',
      join_waitlist: "S'inscrire maintenant",

      station_title: 'Créer un portefeuille',
      station_body:
        'Créez un portefeuille pour gérer vos actifs numériques. Entrez un nom pour votre portefeuille et cliquez sur "Créer".',
      station_name_field: 'Nom du Portefeuille',
      admin_name_field: "Ton nom d'utilisateur",

      waitlist_pending_title: 'Vous êtes sur la liste d attente!',
      waitlist_pending_body:
        'Veuillez attendre l approbation. Vous recevrez un email une fois votre demande approuvée.',
      waitlist_denied_title: 'Vous avez été refusé l accès.',
      waitlist_denied_body:
        'Malheureusement, vous n êtes pas éligible pour rejoindre la liste d attente.',

      waitlist_check_error_title: 'Échec de vérification du statut de la liste d attente',
      waitlist_check_error_body:
        "Échec de la vérification du statut de la liste d'attente, veuillez réessayer.",

      quota_exceed_error_title: 'Quota dépassé',
      quota_exceed_error_body: 'Le nombre maximum de portefeuilles a été atteint.',

      join_station_title: 'Rejoindre un portefeuille existant',
      join_station_body:
        "Contactez le propriétaire pour obtenir l'ID du portefeuille et envoyez-lui votre identité afin qu'un utilisateur puisse être créé pour vous.",
      join_station_canister_id: 'ID du Portefeuille',
      join_station_name: 'Nom du Portefeuille',
      join_station: 'Rejoindre le portefeuille',

      status_starting: 'Initialization, veuillez patienter...',
      status_deploying: 'Deploiement de votre portefeuille au Internet Computer ...',
      status_waiting_for_canister_initialization: 'En attente de la fin du deploiement ...',
      status_creating_initial_account: 'Creation de votre compte initial ...',
      status_completed: "Portefeuille créé avec succès! Vous pouvez maintenant l'utiliser.",
      status_failed: "Échec de l'initialization, veuillez essayer de nouveau.",
    },
    requests: {
      title: 'Demandes',
      transfer_title: 'Demandes de Transfert',
    },
    permissions: {
      title: 'Permissions',
      update_dialog_title: 'Modifier les Permissions',
    },
    request_policies: {
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
      title_not_found_user_identity: 'Vous n êtes pas ajouté au portefeuille',
      subtitle_not_found_user_identity:
        'Contactez le propriétaire du portefeuille pour ajouter un usager pour vous avec votre principal.',

      title_other_station_error: 'Impossible de se connecter au portefeuille',
      subtitle_other_station_error: 'Le portefeuille a retourné l erreur suivante:',

      title_canister_error: 'Impossible de se connecter au portefeuille',
      subtitle_canister_error:
        'Il y a eu une erreur en accédant au portefeuille. Vérifiez votre connection internet et que l ID du portefeuille correspond à un portefeuille valide.',
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
  permissions: {
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
      permission: "Police d'Accés",
      requestpolicy: 'Police de Demande',
      system: 'Système',
      transfer: 'Transfert',
      request: 'Demande',
      addressbook: "Carnet d'Adresses",
      managesysteminfo: 'Gérer les informations système',
      externalcanister: 'Canister Géré',
    },
    actions: {
      list: 'Lister',
      create: 'Créer',
      read: 'Lire',
      update: 'Modifier',
      delete: 'Éffacer',
      transfer: 'Transfert',
      capabilities: 'Capacités',
      systeminfo: 'Information Système',
      systeminfocapabilities: 'Capacités (Actifs Pris en Charge)',
      systeminfoconfig: 'Configuration (Mises à jour, Métriques, Utilisation)',
      managesysteminfo: 'Gérer les informations système (par exemple. nom)',
      systemupgrade: 'Mise à jour du système',
      change: 'Changement',
      fund: 'Financer',
    },
    allow: {
      public: "N'importe qui",
      authenticated: 'Authentifié',
      restricted: 'Restreint',
    },
  },
  request_policies: {
    user_type_select: "Type d'Usager",
    add_rule_label: 'Ajouter un règle +',
    unsupported_specifier: 'Définition de spécificateur non supportée',
    rule_user_specifier: {
      owner: 'Propriétaire',
      requester: 'Demandeur',
      any: "N'importe quel usager",
      group: 'Membre du groupe',
      id: 'Usager spécifique',
    },
    rule: {
      allof: 'Tout les',
      anyof: 'Un des',
      not: 'Aucun des',
      autoapproved: 'Auto-approuvé',
      quorum: 'Quorum',
      quorumpercentage: 'Pourcentage du Quorum',
      allowlistedbymetadata: 'Liste blanche par metadata',
      allowlisted: 'Liste blanche',
    },
    specifier: {
      editpermission: 'Modifier les permissions',
      addusergroup: "Ajouter un groupe d'usagers",
      removerequestpolicy: 'Éffacer une police de demande',
      adduser: 'Ajouter un usager',
      editusergroup: "Modifer le groupe d'usagers",
      removeaddressbookentry: "Éffacer une entrée dans le carnet d'adresses",
      editaddressbookentry: "Modifier une entrée dans le carnet d'adresses",
      addrequestpolicy: 'Ajouter une police de demande',
      systemupgrade: 'Mise à jour du système',
      editrequestpolicy: 'Modifier une police de demande',
      edituser: 'Modifier un usager',
      transfer: 'Transfert',
      editaccount: 'Modifier un compte',
      addaddressbookentry: "Ajouter une entrée dans le carnet d'adresses",
      removeusergroup: "Éffacer un groupe d'usagers",
      addaccount: 'Ajouter un compte',
      managesysteminfo: 'Gérer les informations système',
      changeexternalcanister: 'Modifier un canister',
      fundexternalcanister: 'Financer un canister',
      setdisasterrecovery: 'Définir la récupération après sinistre',
      callexternalcanister: 'Appeler un canister',
      createexternalcanister: 'Créer un canister',
    },
  },
  cycle_obtain_strategies: {
    disabled: 'Désactivé',
    mintfromnativetoken: 'Mint depuis le compte ICP',
  },
};
