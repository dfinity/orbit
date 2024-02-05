export default {
  app: {
    title: '{app} Carteira',
    action_save_failed: 'Erro ao gravar a ação, por favor, tente novamente.',
    action_save_success: 'Ação gravada com sucesso.',
    session_load_error: 'Erro ao inicializar a sessão, por favor, tente novamente.',
    user_id: 'ID do usuário',
    wallets: 'Carteiras',
    confirm: 'Confirmar',
    copied_to_clipboard: 'Texto copiado para a área de transferência.',
    initial_account_name: 'Principal',
    alpha_warning: 'Descubra mais sobre esta versão alfa, por favor, utilize com cuidado.',
    wallet_info_card_title: '{name} Informaçōes',
    wallet_info_card_edit_btn: 'Editar carteira',
    wallet_info_card_remove_btn: 'Remover carteira',
    wallet_info_card_remove_btn_confirm: 'Tem a certeza que pretende remover esta carteira?',
    manage_associated_wallet: 'Gerenciar carteira associada',
    manage_associated_wallet_hint:
      'Qualquer modificação apenas serão aplicadas a sua conta de usuário.',
    user_activities_card_title: 'Atividades do usuário',
    wallet_upgrades_card_title: 'Atualizaçōes de sistema',
    data_load_error: 'Erro ao carregar os dados, por favor, tente novamente.',
    dialog_confirmation_title: 'Confirmação',
    dialog_confirmation_question: 'Tem a certeza que pretende continuar?',
    request_failed_message: 'O pedido falhou, por favor, tente novamente.',
    request_pending_message: 'O seu pedido foi criado e está pendente de aprovação.',
    request_adopted_message: 'Este pedido foi aceite e está sendo processado.',
    request_rejected_message: 'Este pedido foi rejeitado.',
    user_status_active: 'Ativo',
    user_status_inactive: 'Inativo',
    add_new_principal: 'Adicionar novo principal',
    principal_already_added: 'Principal já adicionado.',
    user_associate_principal_warning:
      'Utilize com cuidado. O principal poderá aceder à sua conta e executar ações em seu nome.',
    export_csv: 'Exportar CSV',
    params_parse_error: 'Erro ao analisar os parâmetros, por favor, tente novamente.',
  },
  proposals: {
    title_info_message: 'O título definido pelo requerente.',
    proposed_by: 'Proposto por {name}',
    proposer_id: 'ID do requerente: {id}',
    status: {
      created: 'Pendente',
      cancelled: 'Cancelado',
      adopted: 'Adotado',
      rejected: 'Rejeitado',
      completed: 'Concluído',
      failed: 'Falhou',
      processing: 'Processando',
      scheduled: 'Agendado',
      unknown: 'Desconhecido',
    },
    no_results_found: 'Nenhum resultado encontrado.',
    processing_started_at: 'Processamento iniciado em {dt}',
    processing_completed_at: 'Processamento concluído em {dt}',
    processing_scheduled_at: 'Processamento agendado para {dt}',
    no_cancelled_reason: 'Nenhuma razão especificada.',
    no_failed_reason: 'Nenhuma razão especificada.',
    domains: {
      all: 'Todos',
      accounts: 'Contas',
      address_book: 'Livro de endereços',
      system: 'Sistema',
      transfers: 'Transferências',
      users: 'Usuários',
    },
    types: {
      addusergroup: {
        title: 'Adicionar grupo de usuários',
        request_title: 'Pedido de adição de grupo de usuários',
      },
      addaccount: {
        title: 'Adicionar conta',
        request_title: 'Pedido de adição de conta',
      },
      adduser: {
        title: 'Adicionar usuário',
        request_title: 'Pedido de adição de usuário',
      },
      addaccesspolicy: {
        title: 'Adicionar regra de acesso',
        request_title: 'Pedido de adição de regra de acesso',
      },
      addaddressbookentry: {
        title: 'Adicionar novo endereço',
        request_title: 'Pedido de adição de endereço',
      },
      addproposalpolicy: {
        title: 'Adicionar regra de aprovação',
        request_title: 'Pedido de adição de regra de aprovação',
      },
      removeproposalpolicy: {
        title: 'Remover regra de aprovação',
        request_title: 'Pedido de remoção de regra de aprovação',
      },
      removeaccesspolicy: {
        title: 'Remover regra de acesso',
        request_title: 'Pedido de remoção de regra de acesso',
      },
      removeusergroup: {
        title: 'Remover grupo de usuários',
        request_title: 'Pedido de remoção de grupo de usuários',
      },
      removeaddressbookentry: {
        title: 'Remover endereço',
        request_title: 'Pedido de remoção de endereço',
      },
      changecanister: {
        title: 'Alterar canister',
        request_title: 'Pedido de alteração de canister',
      },
      editusergroup: {
        title: 'Editar grupo de usuários',
        request_title: 'Pedido de edição de grupo de usuários',
      },
      edituser: {
        title: 'Editar usuário',
        request_title: 'Pedido de edição de usuário',
      },
      editaccount: {
        title: 'Editar conta',
        request_title: 'Pedido de edição de conta',
      },
      editaddressbookentry: {
        title: 'Editar endereço',
        request_title: 'Pedido de edição de endereço',
      },
      transfer: {
        title: 'Transferir',
        request_title: 'Pedido de transferência',
      },
      editproposalpolicy: {
        title: 'Editar regra de aprovação',
        request_title: 'Pedido de edição de regra de aprovação',
      },
      unknown: {
        title: 'Desconhecido',
        request_title: 'Pedido desconhecido',
      },
    },
  },
  login: {
    signin_slogan: 'Conecte-se para gerir a sua carteira de ativos crypto',
    auth_failed: 'Login falhou, tente novamente',
  },
  not_found: {
    title: 'Ups, 404',
    description: 'A página que está a tentar aceder não existe.',
    btn_back: 'Voltar ao início',
  },
  slogans: {
    elevate_to_orbit: {
      main: 'Se eleve à {term1}, {term2}',
      term1: 'Órbita',
      term2: 'onde a segurança e a conveniência se alinham',
    },
    institutions_multi_custody: {
      main: 'Onde as {term1} e as carteiras {term2} se alinham',
      term1: 'Instituições',
      term2: 'Multi-custódia',
    },
  },
  home: {
    welcome_back: 'Boas-vindas de novo',
    notifications: {
      none: 'Não tem notificaçōes por ler',
      some: 'Tem {count} notificaçōes por ler',
    },
  },
  footer: {
    copyright: '© 2023 - DFINITY Foundation',
    github: {
      description: 'Código-fonte',
    },
  },
  settings: {
    subtitle: 'Configure as preferências e gerencie as identidades associadas à sua conta.',
    edit_success: 'As informaçōes da sua conta foram alteradas com sucesso.',
    load_failed: 'Falha ao procurar as informaçōes da sua conta, por favor, tente novament.',
  },
  wallets: {
    add_account_proposal_saved: 'Pedido de criação de conta enviado',
    edit_account_proposal_saved: 'Pedido de atualização de conta enviado',
    pending_account_creation_subtitle: 'Criação de conta pendente ...',
    proposal_failed_to_save: 'Erro ao gravar a proposta.',
    notification_failed_to_save: 'Erro ao gravar a notificação.',
    no_accounts: 'Nenhuma conta disponível.',
    pending_proposals: 'Propostas pendentes',
    pending_requests: 'Pedidos pendentes',
    user_copied_to_clipboard: 'Id de utilizador copiado.',
    account_address_copied_to_clipboard: 'Endereço da conta copiado.',
    load_error: 'Erro ao carregar as informaçōes das carteiras, por favor, tente novamente.',
    wallet_nr_title: '#{nr} Carteira',
    load_error_withdraw_requests: 'Erro ao carregar os pedidos de retirada.',
    no_wallets: 'Nenhuma carteira disponível.',
    user_load_error: 'Erro ao carregar a sua conta de utilizador.',
    no_wallet_user: 'Nenhum utilizador de carteira disponível.',
    please_register_to_continue: 'Por favor registe-se para continuar.',
    private_account: 'Conta privada',
    joint_account: 'Conta conjunta',
    policy: 'Regra',
    policy_misconfigured: 'As regras da sua conta estão mal configuradas.',
    policy_config_unavailable: 'Configuração da regra indisponível.',
    policy_fixed_approval_threshold_desc: 'Número fixo de aprovações para operaraçōes',
    policy_variable_approval_threshold_desc: 'Percentual de aprovações para operaraçōes',
    policies: {
      VariableApprovalThreshold: 'Aprovação por porcentagem',
      FixedApprovalThreshold: 'Aprovação por votos fixos',
    },
    proposals: {
      transfer: {
        title: 'Approvar transferência',
      },
    },
    no_deposit_found_search: 'Nenhum depósito encontrado para a pesquisa.',
    no_withdrawal_found_search: 'Nenhuma retirada encontrada para a pesquisa.',
    no_withdraw_request_found_search: 'Nenhum pedido de retirada encontrado para a pesquisa.',
    add_wallet_list_item: 'Adicionar carteira existente',
    add_wallet_dialog_title: 'Adicionar carteira',
    add_wallet_dialog_already_added: 'Esta carteira já foi adicionada.',
  },
  terms: {
    deposits: 'Depósitos',
    wallet: 'Carteira',
    all_done: 'Tudo pronto',
    destination_address: 'Endereço de destino',
    search: 'Procurar',
    proposals: 'Propostas',
    withdraw_requests: 'Pedidos de retirada',
    approve: 'Aprovar',
    id: 'ID',
    submit: 'Submeter',
    save: 'Salvar',
    type: 'Tipo',
    view: 'Ver',
    from: 'De',
    clear: 'Limpar',
    filters: 'Filtros',
    until: 'Até',
    approved: 'Aprovado',
    confirm: 'Confirmar',
    cancel: 'Cancelar',
    see_all: 'Ver todos',
    requests: 'Pedidos',
    reject: 'Rejeitar',
    statuses: 'Estados',
    created: 'Criado',
    expires: 'Expira',
    rejected: 'Rejeitado',
    to: 'Para',
    requested: 'Requerido',
    summary: 'Resumo',
    review: 'Validar',
    yes: 'Sim',
    reset: 'Reiniciar',
    no: 'Não',
    edit: 'Editar',
    amount: 'Valor',
    account: 'Conta',
    send: 'Enviar',
    unknown: 'Desconhecido',
    open: 'Abrir',
    created_at: 'Criado em',
    expires_at: 'Expira em',
    asset: 'Ativo',
    failed: 'Erro',
    cancelled: 'Cancelado',
    user: 'Utilizador',
    user_id: 'Identificador de utilizador',
    login: 'Entrar',
    logout: 'Sair',
    signin: 'Entrar',
    create: 'Criar',
    signout: 'Sair',
    new_transfer: 'Nova transferência',
    scheduled: 'Agendado',
    processing: 'Processando',
    anonymous: 'Anónimo',
    accounts: 'Contas',
    token: 'Token',
    new_account: 'Criar conta',
    edit_account: 'Editar conta',
    policies: 'Regras',
    balance: 'Saldo',
    address: 'Endereço',
    transfers: 'Transferências',
    withdrawals: 'Withdrawals',
    transactions: 'Transaçōes',
    address_book: 'Endereços',
    completed: 'Realizado',
    pending: 'Pendente',
    name: 'Nome',
    new_withdraw: 'Nova retirada',
    settings: 'Configuraçōes',
    close: 'Fechar',
    general: 'Geral',
    add: 'Adicionar',
    remove: 'Remover',
    owners: 'Proprietários',
    user_name: 'Nome de utilizador',
    users: 'Usuários',
    wallet_name: 'Nome da carteira',
    identity_name: 'Nome da identidade',
    canister_id: 'Canister ID',
    principal: 'Principal',
    status: 'Estado',
    control_panel: 'Painel de controle',
    confirmed: 'Confirmado',
    unconfirmed: 'Não confirmado',
    main: 'Principal',
    user_group: 'Grupo de usuário',
    user_groups: 'Grupos de usuários',
  },
  account_page: {
    not_found_title: 'Conta não encontrada',
    not_found_description: 'Não foi possível aceder a conta requisitada.',
    not_found_btn: 'Voltar as contas',
  },
  forms: {
    create: 'Criar',
    edit: 'Editar',
    wallets: 'Carteiras ({min}/{max})',
    identities: 'Identidades ({min}/{max})',
    save_changes: 'Gravar alteraçōes',
    rules: {
      required: 'Este campo é obrigatório.',
      maxLength: 'O tamanho máximo do {field} são {max} caractéres.',
      validPrincipal: 'Este campo deve conter um principal válido.',
      validCanisterId: 'Este campo deve conter um ID de canister válido.',
      validUuidV4: 'Este campo deve conter um UUID v4 válido.',
      duplicate: 'Este campo deve ser único.',
      validTokenAmount: 'Este campo deve conter um valor de token válido.',
    },
  },
  navigation: {
    accounts: 'Contas',
    address_book: 'Endereços',
    users: 'Usuários',
    settings: 'Configuraçōes',
    user_groups_permissions: 'Grupos de usuários & Permissões',
    administration: 'Administração',
    add_another_wallet: 'Adicionar outra carteira',
    account_info_settings: 'Informaçōes da conta & Configuraçōes',
    login: 'Entrar',
    logout: 'Sair',
    proposals: 'Pedidos',
    transfer_proposals: 'Pedidos de transferência',
    permissions: 'Permissões',
  },
  pages: {
    user_settings: {
      title: 'Informaçōes do usuário & Configuraçōes',
      subtitle: 'Configure as preferências e gerencie a sua conta.',
    },
    administration: {
      title: 'Administração da carteira',
    },
    user_groups: {
      title: 'Grupos de usuários',
      btn_new_group: 'Criar grupo',
      btn_manage_permissions: 'Gerir permissōes',
      error_loading_user_groups:
        'Erro ao carregar os grupos de usuários, por favor, tente novamente.',
      btn_edit_title: 'Editar grupo de usuários',
      create_new_group_title: 'Criar novo grupo de usuários',
    },
    users: {
      title: 'Usuários',
      btn_new_user: 'Criar usuário',
      create_new_user_title: 'Criar novo usuário',
      btn_edit_title: 'Editar usuário',
      error_fetching_users: 'Erro ao carregar os usuários, por favor, tente novamente.',
    },
    initialization: {
      status_starting: 'Inicializando, por favor, aguarde ...',
      status_deploying: 'Instalando a sua carteira no Internet Computer ...',
      status_waiting_for_canister_initialization: 'Aguardando a instalação ser concluída ...',
      status_creating_initial_account: 'Criando a sua primeira conta ...',
      status_completed:
        'A sua carteira foi instalada com sucesso, por favor, aguarde enquanto é redirecionado ...',
      status_failed: 'Falha com a inicialização, por favor, tente novamente.',
    },
    proposals: {
      title: 'Pedidos',
      transfer_title: 'Pedidos de transferência',
    },
    permissions: {
      title: 'Permissões',
      update_dialog_title: 'Atualizar permissões',
    },
  },
  session: {
    expired_dialog_title: 'Sua sessão expirou',
    expired_dialog_content: 'Você deve se reautenticar para continuar.',
    expired_dialog_btn: 'Reautenticar',
  },
  permissions: {
    resource_title: 'Recurso',
    group_members_title: 'Membros do grupo',
    specific_users_title: 'Usuários específicos',
    everyone_title: 'Todos',
    resources: {
      account: 'Conta',
      user: 'Usuário',
      usergroup: 'Grupo de usuários',
      accesspolicy: 'Regra de acesso',
      proposalpolicy: 'Regra para pedidos',
      canistersettings: 'Configuraçōes do canister',
      changecanister: 'Alterar canister',
      transfer: 'Transfência',
      proposal: 'Pedido',
      addressbook: 'Livro de endereços',
    },
    actions: {
      list: 'Listar',
      create: 'Criar',
      read: 'Ler',
      update: 'Atualizar',
      delete: 'Remover',
      readpublicconfig: 'Ler configuração pública',
      readsensitiveconfig: 'Ler configuração sensível',
    },
  },
};
