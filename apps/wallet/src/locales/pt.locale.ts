export default {
  app: {
    title: '{app} Carteira',
    action_save_failed: 'Erro ao gravar a ação, por favor, tente novamente.',
    action_save_success: 'Ação gravada com sucesso.',
    session_load_error: 'Erro ao inicializar a sessão, por favor, tente novamente.',
    test_environment_warning_banner: {
      main: 'ATENÇÃO: Ambiente de teste.',
      info: 'Funcionalidades e dados instáveis.',
    },
    api_compatibility_error:
      'Falha ao verificar a compatibilidade da sua carteira, pode encontrar problemas.',
    stations: 'Carteiras',
    confirm: 'Confirmar',
    copied_to_clipboard: 'Texto copiado para a área de transferência.',
    initial_account_name: 'Principal',
    station_info_card_title: '{name} Informaçōes',
    station_info_card_edit_btn: 'Preferências',
    station_info_card_edit_hint: 'Edite suas preferências e configurações para esta carteira.',
    station_info_card_remove_btn: 'Remover carteira',
    station_info_card_remove_btn_confirm: 'Tem a certeza que pretende remover esta carteira?',
    manage_associated_station: 'Gerenciar carteira associada',
    manage_associated_station_hint:
      'Qualquer modificação apenas serão aplicadas a sua conta de usuário.',
    user_activities_card_title: 'Atividades do usuário',
    station_upgrades_card_title: 'Atualizaçōes de sistema',
    data_load_error: 'Erro ao carregar os dados, por favor, tente novamente.',
    dialog_confirmation_title: 'Confirmação',
    dialog_confirmation_question: 'Tem a certeza que pretende continuar?',
    request_failed_message: 'O pedido falhou, por favor, tente novamente.',
    request_pending_message: 'O seu pedido foi criado e está pendente de aprovação.',
    request_approved_message: 'Este pedido foi approvado e está sendo processado.',
    request_rejected_message: 'Este pedido foi rejeitado.',
    user_status_active: 'Ativo',
    user_status_inactive: 'Inativo',
    add_new_identity: 'Adicionar nova identidade',
    principal_already_added: 'Principal já adicionado.',
    user_associate_identity_warning:
      'Utilize com cuidado. A identidade poderá aceder à sua conta e executar ações em seu nome.',
    export_csv: 'Exportar CSV',
    params_parse_error: 'Erro ao analisar os parâmetros, por favor, tente novamente.',
    submit_upgrade: 'Submeter atualização',
    canister_upgrade_target: 'Canister de destino',
    canister_wasm_module: 'Módulo WASM do canister',
    canister_upgrade_args_input: 'Argumentos de atualização do canister (opcional)',
    canister_upgrade_args_input_hint: 'Apenas aceite no formato hexadecimal.',
    search_items: 'Procurar items',
    search_users: 'Procurar usuários',
    search_user_groups: 'Procurar grupos de usuários',
    search_accounts: 'Procurar contas',
    destination_source: 'Destino / Origem',
    amount_token: 'Quantidade, {token}',
    no_transfers: 'Nenhuma transferência disponível.',
    address_book_entry: 'Entrada do livro de endereços',
    notifications_panel_title: 'Notificações',
    notifications_panel_no_results: 'Sem notificações por ler.',
    notifications_panel_read_all: 'Ler todas',
    notifications_request_failed: 'Pedido falhou: {reason}',
    btn_home_back: 'Voltar ao início',
    no_download_available: 'Nenhum download disponível.',
    failed_to_download_item: 'Falha ao baixar {item}, por favor, tente novamente.',
    download_error: 'Erro ao baixar o ficheiro, por favor, tente novamente.',
    leave_page_warning:
      'Tem a certeza que pretende sair? As alterações não gravadas serão perdidas.',
    loading_details: 'Carregando detalhes ...',
    account_dialog_create_new_title: 'Criar nova conta',
    account_dialog_view_title: 'Conta',
    account_dialog_access_read: 'Ler',
    account_dialog_access_read_hint: 'Acesso somente leitura à conta.',
    account_dialog_access_configuration: 'Alterar configurações da conta',
    account_dialog_access_configuration_hint:
      'Acesso para alterar as configurações da conta, como nome da conta, políticas, etc.',
    account_dialog_access_transfer: 'Transferir fundos',
    account_dialog_access_transfer_hint: 'Acesso para transferir fundos da conta.',
    account_dialog_request_policy_configuration: 'Alterar configurações da conta',
    account_dialog_request_policy_configuration_hint:
      'A política que precisa ser aprovada para alterar as configurações da conta.',
    account_dialog_request_policy_transfer: 'Transferir fundos',
    account_dialog_request_policy_transfer_hint:
      'A política que precisa ser aprovada para transferir fundos.',
    request_policy_rule_builder_no_rule: 'Nenhum critério definido.',
  },
  alpha_warning: {
    version: 'Esta é uma versão alfa.',
    caution: 'Utilize com cuidado.',
  },
  sidebar: {
    highlights: {
      main: 'Carteira Trustless {line1} {line2} {line3}',
      line3: 'Multichain',
      line1: 'Ativos Digitais',
      line2: 'Multi-Custódia',
    },
  },
  change_canister: {
    targets: {
      upgradestation: 'Carteira',
      upgradeupgrader: 'Atualizador',
    },
  },
  blockchains: {
    icp: {
      name: 'Internet Computer',
      standards: {
        native: 'Nativo',
      },
    },
    eth: {
      name: 'Ethereum',
      standards: {
        native: 'Nativo',
      },
    },
    btc: {
      name: 'Bitcoin',
      standards: {
        native: 'Nativo',
      },
    },
  },
  requests: {
    unsupported_operation: 'Operação não suportada',
    title_info_message: 'O título definido pelo requerente.',
    requested_by: 'Requisitado por {name}',
    requester_id: 'ID do requerente: {id}',
    status: {
      created: 'Pendente',
      cancelled: 'Cancelado',
      approved: 'Aprovado',
      rejected: 'Rejeitado',
      completed: 'Concluído',
      failed: 'Falhou',
      processing: 'Processando',
      scheduled: 'Agendado',
      unknown: 'Desconhecido',
    },
    approvals: 'Aprovações',
    requester_auto_approval: 'Pedido aprovado automaticamente pelo requerente',
    approvals_and_evaluation: 'Aprovações e regras',
    failure_title: 'Falha na execução do pedido',
    failure_reason_unknown: 'Pedido falhou por uma razão não especificada.',
    comment_optional: 'Comentário (opcional)',
    no_results_found: 'Nenhum resultado encontrado.',
    no_more_requests_to_approve: 'Não há mais pedidos para aprovar.',
    load_next: 'Carregar próximo',
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
    download: {
      user_group: 'Grupos de usuários',
      user: 'Usuários',
      account: 'Contas',
      permission: 'Regras de acesso',
      request_policy: 'Regras de aprovação',
      address_book_entry: 'Livro de endereços',
      change_canister: 'Atualizações de sistema',
      transfer: 'Transferências',
      external_canister: 'Canister gerenciado',
      system_info: 'Informações do sistema',
    },
    headers: {
      id: 'ID',
      status: 'Estado',
      status_reason: 'Razão do estado',
      created: 'Criado',
      expires: 'Expira',
      operation_type: 'Tipo de operação',
      requester: 'Requerente',
      details: 'Detalhes',
      account_id: 'ID da conta',
      account_name: 'Nome da conta',
      token: 'Token',
      address: 'Endereço',
      user_id: 'ID do usuário',
      user_name: 'Nome do usuário',
      group_id: 'ID do grupo',
      group_name: 'Nome do grupo',
      address_book_id: 'ID do livro de endereços',
      blockchain: 'Blockchain',
      address_owner: 'Proprietário do endereço',
      policy_id: 'ID da regra',
      change_target: 'Alvo da atualização',
      wasm_checksum: 'Checksum do módulo WASM',
      from_account: 'De conta',
      to: 'Para',
      amount: 'Quantidade',
      fee: 'Taxa',
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
      addaddressbookentry: {
        title: 'Adicionar novo endereço',
        request_title: 'Pedido de adição de endereço',
      },
      addrequestpolicy: {
        title: 'Adicionar regra de aprovação',
        request_title: 'Pedido de adição de regra de aprovação',
      },
      removerequestpolicy: {
        title: 'Remover regra de aprovação',
        request_title: 'Pedido de remoção de regra de aprovação',
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
      editpermission: {
        title: 'Editar permissão',
        request_title: 'Pedido de alteração de permissão',
      },
      editusergroup: {
        title: 'Editar grupo de usuários',
        request_title: 'Pedido de alteração de grupo de usuários',
      },
      edituser: {
        title: 'Editar usuário',
        request_title: 'Pedido de alteração de usuário',
      },
      editaccount: {
        title: 'Editar conta',
        request_title: 'Pedido de alteração de conta',
      },
      editaddressbookentry: {
        title: 'Editar endereço',
        request_title: 'Pedido de alteração de endereço',
      },
      transfer: {
        title: 'Transferir',
        request_title: 'Pedido de transferência',
      },
      editrequestpolicy: {
        title: 'Editar regra de aprovação',
        request_title: 'Pedido de alteração de regra de aprovação',
      },
      managesysteminfo: {
        title: 'Gerir informações do sistema',
        request_title: 'Pedido de alteração de informações do sistema',
      },
      unknown: {
        title: 'Desconhecido',
        request_title: 'Pedido desconhecido',
      },
    },
    evaluation: {
      acceptance_rules: 'Regras de aceitação',
      show_acceptance_rules: 'Mostrar regras de aceitação',
      hide_acceptance_rules: 'Ocultar regras de aceitação',
      allof_rule: 'Todas as seguintes {n} regras:',
      anyof_rule: 'Qualquer das seguintes {n} regras:',
      not_rule: 'Não deve passar:',
      allowlisted_rule: 'O endereço de destino está no Livro de Endereços',
      not_found_in_allow_list: 'Não está no Livro de Endereços',
      found_in_allow_list: 'No Livro de Endereços',
      allowlisted_with_metadata_rule: 'O endereço de destino tem metadados no Livro de Endereços',
      allow_list_metadata_not_found: 'Não encontrado {metadata}',
      allow_list_metadata_found: 'Encontrado: {metadata}',
      quorum_rule: '1 assinatura de aprovação mínima | {n} assinaturas de aprovação mínimas',
      quorum_percentage_rule:
        '1 assinatura de aprovação mínima | {n} assinaturas de aprovação mínimas',
      approval_summary_approved: 'Aprovado com {n} para {m} contra',
      approval_summary_rejected: 'Rejeitado com {n} para {m} contra',
      approval_summary_pending: 'Pendente com {n} para {m} contra',
      approval_comments: '1 comentário | {n} comentários',
      auto_approved: 'Auto-aprovado',
      pending: 'Pendente',
      rejected: 'Rejeitado',
      approved: 'Aprovado',

      summary_approved:
        'Pedido aprovado para a seguinte razão: | Pedido aprovado para as seguintes razões:',
      summary_rejected:
        'Pedido rejeitado para a seguinte razão: | Pedido rejeitado para as seguintes razões:',
      summary_pending:
        'Pedido pendente para a seguinte razão: | Pedido pendente para as seguintes razões:',
      approved_reason_approval_quorum: 'limite de aprovação do usuário atingido',
      approved_reason_allowlist: 'endereço de destino encontrado no livro de endereços',
      approved_reason_allowlist_metadata:
        'endereço de destino tinha metadados no livro de endereços',
      reason_auto_approved: 'pedido foi auto-aprovado',

      rejected_reason_approval_quorum: 'limite de aprovação do usuário não atingido',
      rejected_reason_allowlist: 'endereço de destino não encontrado no livro de endereços',
      rejected_reason_allowlist_metadata:
        'endereço de destino não tinha metadados no livro de endereços',

      pending_reason_approval_quorum: 'aprovação do usuário pendente',
      pending_reason_allowlist: 'endereço de destino no livro de endereços',
      pending_reason_allowlist_metadata: 'endereço de destino no livro de endereços com metadados',
    },
  },
  landing: {
    title: 'Gestão Multichain',
    subtitle: 'Uma Plataforma, Supervisão Completa',
    description:
      'Orbit simplifica a gestão de ativos on-chain para empresas, DAOs e equipes, consolidando controle e visibilidade em uma única plataforma intuitiva.',
    connect_title: 'Conecte-se de forma segura para gerenciar seus ativos digitais',
    connect_btn: 'Conectar com Internet Identity',
    connect_error: 'Falha ao conectar, por favor tente novamente.',
  },
  home: {
    welcome_back: 'Boas-vindas de novo',
    notifications: {
      none: 'Não tem notificaçōes por ler',
      some: 'Tem {count} notificaçōes por ler',
    },
  },
  footer: {
    copyright: '© 2024 - DFINITY Foundation',
    github: {
      description: 'Código-fonte',
    },
  },
  settings: {
    subtitle: 'Configure as preferências e gerencie as identidades associadas à sua conta.',
    edit_success: 'As informaçōes da sua conta foram alteradas com sucesso.',
    load_failed: 'Falha ao procurar as informaçōes da sua conta, por favor, tente novament.',
  },
  stations: {
    add_account_request_saved: 'Pedido de criação de conta enviado',
    edit_account_request_saved: 'Pedido de atualização de conta enviado',
    pending_account_creation_subtitle: 'Criação de conta pendente ...',
    request_failed_to_save: 'Erro ao gravar o pedido.',
    notification_failed_to_save: 'Erro ao gravar a notificação.',
    no_accounts: 'Nenhuma conta disponível.',
    pending_requests: 'Pedidos pendentes',
    user_copied_to_clipboard: 'Id de utilizador copiado.',
    account_address_copied_to_clipboard: 'Endereço da conta copiado.',
    load_error: 'Erro ao carregar as informaçōes das carteiras, por favor, tente novamente.',
    station_nr_title: '#{nr} Carteira',
    load_error_withdraw_requests: 'Erro ao carregar os pedidos de retirada.',
    no_stations: 'Nenhuma carteira disponível.',
    user_load_error: 'Erro ao carregar a sua conta de utilizador.',
    no_station_user: 'Nenhum utilizador de carteira disponível.',
    please_register_to_continue: 'Por favor registe-se para continuar.',
    private_account: 'Conta privada',
    joint_account: 'Conta conjunta',
    policy: 'Regra',
    policy_misconfigured: 'As regras da sua conta estão mal configuradas.',
    policy_config_unavailable: 'Configuração da regra indisponível.',
    policy_fixed_approval_threshold_desc: 'Número fixo de aprovações para operaraçōes',
    policy_variable_approval_threshold_desc: 'Percentual de aprovações para operaraçōes',
    requests: {
      transfer: {
        title: 'Approvar transferência',
      },
    },
    no_deposit_found_search: 'Nenhum depósito encontrado para a pesquisa.',
    no_withdrawal_found_search: 'Nenhuma retirada encontrada para a pesquisa.',
    no_withdraw_request_found_search: 'Nenhum pedido de retirada encontrado para a pesquisa.',
    add_station_list_item: 'Adicionar carteira',
  },
  terms: {
    deposits: 'Depósitos',
    station: 'Carteira',
    all_done: 'Tudo pronto',
    destination_address: 'Endereço de destino',
    station_id: 'ID da carteira',
    search: 'Procurar',
    details: 'Detalhes',
    requests: 'Propostas',
    withdraw_requests: 'Pedidos de retirada',
    approve: 'Aprovar',
    identity: 'Identidade',
    overriden: 'Substituído',
    id: 'ID',
    any: 'Qualquer',
    comment_optional: 'Comentário (opcional)',
    everyone: 'Todos',
    action: 'Ação',
    previous: 'Anterior',
    next: 'Próximo',
    back: 'Voltar',
    wasm: 'Wasm',
    download: 'Baixar',
    arg: 'Arg',
    target: 'Alvo',
    permissions: 'Permissões',
    approval_policies: 'Políticas de Aprovação',
    upgrader: 'Atualizador',
    resource: 'Recurso',
    submit: 'Submeter',
    save: 'Salvar',
    type: 'Tipo',
    identities: 'Identidades',
    view: 'Ver',
    from: 'De',
    specifier: 'Especificador',
    clear: 'Limpar',
    filters: 'Filtros',
    until: 'Até',
    configuration: 'Configuração',
    approved: 'Aprovado',
    confirm: 'Confirmar',
    cancel: 'Cancelar',
    see_all: 'Ver todos',
    min: 'Mínimo',
    rule: 'Regra',
    blockchain: 'Blockchain',
    address_owner: 'Proprietário do endereço',
    checksum: 'Checksum',
    reject: 'Rejeitar',
    metadata: 'Metadados',
    none: 'Nenhum',
    statuses: 'Estados',
    created: 'Criado',
    expires: 'Expira',
    rejected: 'Rejeitado',
    to: 'Para',
    request: 'Pedido',
    requested: 'Requerido',
    summary: 'Resumo',
    review: 'Validar',
    of: 'de',
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
    addresses: 'Endereços',
    token: 'Token',
    new_account: 'Criar conta',
    request_policy: 'Regra de aprovação',
    edit_account: 'Editar conta',
    new_address: 'Novo endereço',
    policies: 'Regras',
    balance: 'Saldo',
    address: 'Endereço',
    transfers: 'Transferências',
    key: 'Chave',
    value: 'Valor',
    withdrawals: 'Withdrawals',
    transactions: 'Transaçōes',
    address_book: 'Endereços',
    completed: 'Realizado',
    invalid: 'Inválido',
    pending: 'Pendente',
    name: 'Nome',
    new_withdraw: 'Nova retirada',
    settings: 'Configuraçōes',
    close: 'Fechar',
    transfer: 'Transferência',
    general: 'Geral',
    time: 'Horário',
    add: 'Adicionar',
    remove: 'Remover',
    owners: 'Proprietários',
    user_name: 'Nome de utilizador',
    users: 'Usuários',
    station_name: 'Nome da carteira',
    identity_name: 'Nome da identidade',
    canister_id: 'Canister ID',
    principal: 'Principal',
    status: 'Estado',
    control_panel: 'Painel de controle',
    confirmed: 'Confirmado',
    total: 'Total',
    unconfirmed: 'Não confirmado',
    main: 'Principal',
    user_group: 'Grupo de usuário',
    user_groups: 'Grupos de usuários',
    all: 'Todos',
    subset: 'Subconjunto',
    skip: 'Saltar',
    version: 'Versão',
    continue: 'Continuar',
  },
  forms: {
    create: 'Criar',
    edit: 'Editar',
    stations: 'Carteiras ({min}/{max})',
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
      requiredIntNumber: 'Este campo deve conter um número inteiro.',
      intNumberRange: '{field} deve estar entre {min} e {max}.',
      validEmail: 'Este campo deve conter um email válido.',
    },
  },
  navigation: {
    home: 'Início',
    accounts: 'Contas',
    address_book: 'Endereços',
    users: 'Usuários',
    settings: 'Configuraçōes',
    user_groups_permissions: 'Grupos de usuários & Permissões',
    administration: 'Administração',
    add_another_station: 'Adicionar outra carteira',
    account_info_settings: 'Informaçōes da conta & Configuraçōes',
    login: 'Entrar',
    logout: 'Sair',
    requests: 'Pedidos',
    transfer_requests: 'Pedidos de transferência',
    permissions: 'Permissões',
    request_policies: 'Regras de aprovação',
  },
  pages: {
    accounts: {
      title: 'Contas',
      btn_new_transfer: 'Nova transferência',
      btn_upload_csv: 'Carregar CSV',
      error_fetching_account: 'Erro ao carregar as conta, por favor, tente novamente.',
    },
    account: {
      not_found: 'Conta não encontrada',
      not_found_description: 'A conta que está a tentar aceder não existe.',
      csv_transfer_subtitle: 'Carregar um ficheiro CSV para transferir fundos para várias contas.',
      csv_transfer_file_format_hint: 'O ficheiro CSV deve conter as colunas "{to}" e "{amount}".',
      csv_transfer_file_column_to: 'Para',
      csv_transfer_file_column_amount: 'Quantidade',
      csv_transfer_file_rows_title: 'Transferências para criar: {count}',
      csv_ignored_transfers_hint: 'Transferências com erros serão ignoradas.',
      csv_transfer_failed: 'Error ao processar transferências, por favor, tente novamente.',
      csv_download_invalid: 'Baixar erros',
    },
    address_book: {
      title: 'Livro de endereços',
      btn_new_entry: 'Nova entrada',
      no_results_found: 'Nenhum resultado encontrado.',
      error_fetching_address_book:
        'Erro ao carregar o livro de endereços, por favor, tente novamente.',
    },
    user_settings: {
      title: 'Informaçōes do usuário & Configuraçōes',
      subtitle: 'Configure as preferências e gerencie a sua conta.',
    },
    administration: {
      title: 'Administração',
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
    add_station: {
      initialization_title: 'Bem-vindo! Como você gostaria de se juntar à Orbit?',
      add_station_title: 'Como você gostaria de adicionar uma carteira?',

      option_join_existing_station: 'Junte-se a uma carteira existente',
      option_deploy_new_station: 'Crie a sua própria carteira',
      join_station_title: 'Junte-se a uma carteira existente',
      join_station_body:
        'Entre em contato com o proprietário para obter o ID da Carteira e envie a eles sua identidade para que um usuário possa ser criado para você.',
      join_station_canister_id: 'ID da carteira',
      join_station_name: 'Nome da carteira',
      join_station: 'Junte-se a carteira',

      station_title: 'Crie a sua própria carteira',
      station_body:
        'Crie a sua própria carteira e adicione usuários para gerenciar as suas contas e ativos digitais.',
      station_name_field: 'Nome da carteira',
      admin_name_field: 'Seu nome de usuário',

      check_permissions_title: 'Verificando o estado da lista de espera ...',
      join_waitlist_title: 'Junte-se à lista de espera',
      join_waitlist_body:
        'Junte-se à lista de espera da Orbit! Insira o seu email para obter acesso antecipado e atualizações exclusivas. A sua jornada começa agora.',
      join_waitlist_email_field: 'Insira o seu endereço de email',
      join_waitlist: 'Inscreva-se agora',

      waitlist_pending_title: 'Você está na lista de espera!',
      waitlist_pending_body:
        'Por favor, aguarde a aprovação. Você receberá um email assim que o seu pedido for aprovado.',
      waitlist_denied_title: 'Você foi negado o acesso.',
      waitlist_denied_body: 'Infelizmente, você não é elegível para se juntar à lista de espera.',

      waitlist_check_error_title: 'Falha ao verificar o estado da lista de espera',
      waitlist_check_error_body:
        'Falha ao verificar o estado da lista de espera, por favor, tente novamente.',

      quota_exceed_error_title: 'Limite de carteiras excedido',
      quota_exceed_error_body: 'Você atingiu o limite de carteiras que pode criar.',

      status_starting: 'Inicializando, por favor, aguarde ...',
      status_deploying: 'Instalando a sua carteira no Internet Computer ...',
      status_waiting_for_canister_initialization: 'Aguardando a instalação ser concluída ...',
      status_creating_initial_account: 'Criando a sua primeira conta ...',
      status_completed:
        'A sua carteira foi instalada com sucesso, por favor, aguarde enquanto é redirecionado ...',
      status_failed: 'Falha com a inicialização, por favor, tente novamente.',
    },
    requests: {
      title: 'Pedidos',
      transfer_title: 'Pedidos de transferência',
    },
    permissions: {
      title: 'Permissões',
      update_dialog_title: 'Atualizar permissões',
    },
    request_policies: {
      title: 'Regras de aprovação',
      create_label: 'Criar Regra',
      dialog_title: 'Regra',
    },
    not_found: {
      title: 'Ups, 404',
      subtitle: 'A página que está a tentar aceder não existe.',
    },
    unauthorized: {
      title: 'Acesso não autorizado',
      subtitle: 'Você não tem permissão para aceder a esta página.',
    },
    disconnected: {
      title_not_found_user_identity: 'Você não está adicionado à carteira',
      subtitle_not_found_user_identity:
        'Contacte o proprietário da carteira para adicionar um usuário para si com o seu principal.',

      title_other_station_error: 'Não é possível conectar à carteira',
      subtitle_other_station_error: 'A carteira retornou o seguinte erro:',

      title_canister_error: 'Não é possível conectar à carteira',
      subtitle_canister_error:
        'Houve um erro ao aceder à carteira. Verifique a sua conexão à internet e se o ID da carteira corresponde a uma carteira válida.',
    },
    error: {
      title: 'Erro',
      subtitle: 'Ocorreu um erro, por favor, tente novamente.',
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
    individual_resources_title: 'Acesso aos recursos individuais',
    select_resource: 'Selecione o tipo de recurso',
    resources: {
      account: 'Conta',
      user: 'Usuário',
      usergroup: 'Grupo de usuários',
      permission: 'Regra de acesso',
      requestpolicy: 'Regra para pedidos',
      system: 'Sistema',
      changecanister: 'Alterar canister',
      transfer: 'Transfência',
      request: 'Pedido',
      addressbook: 'Livro de endereços',
      managesysteminfo: 'Gerir informações do sistema',
    },
    actions: {
      list: 'Listar',
      create: 'Criar',
      read: 'Ler',
      update: 'Atualizar',
      delete: 'Remover',
      transfer: 'Transferência',
      capabilities: 'Capacidades',
      systeminfo: 'Informações do sistema',
      systeminfocapabilities: 'Capacidades (Ativos Suportados)',
      systeminfoconfig: 'Configuração (Atualizações, Métricas, Uso)',
      managesysteminfo: 'Gerir Informações do Sistema (e.g. nome)',
    },
    allow: {
      public: 'Acesso público',
      authenticated: 'Autenticado',
      restricted: 'Restrito',
    },
  },
  request_policies: {
    user_type_select: 'Tipo de usuário',
    add_rule_label: 'Adicionar regra +',
    unsupported_specifier: 'Especificador não suportado',
    rule_user_specifier: {
      owner: 'Proprietário',
      requester: 'Requerente',
      any: 'Qualquer usuário',
      group: 'Grupo de usuários',
      id: 'Usuário específico',
    },
    rule: {
      allof: 'todos',
      anyof: 'qualquer',
      not: 'nenhum',
      autoapproved: 'Aprovado automáticamente',
      quorum: 'Quórum',
      quorumpercentage: 'Percentual de quórum',
      allowlistedbymetadata: 'Lista branca por metadados',
      allowlisted: 'Lista branca',
    },
    specifier: {
      editpermission: 'Editar permissão',
      addusergroup: 'Adicionar grupo de usuários',
      removerequestpolicy: 'Remover regra',
      adduser: 'Adicionar usuário',
      editusergroup: 'Editar grupo de usuários',
      removeaddressbookentry: 'Remover entrada do livro de endereços',
      editaddressbookentry: 'Editar entrada do livro de endereços',
      addrequestpolicy: 'Adicionar regra',
      changecanister: 'Alterar canister',
      editrequestpolicy: 'Editar permissão',
      edituser: 'Editar usuário',
      transfer: 'Transferência',
      editaccount: 'Editar conta',
      addaddressbookentry: 'Adicionar entrada no livro de endereços',
      removeusergroup: 'Remove grupo de usuários',
      addaccount: 'Adicionar conta',
      managesysteminfo: 'Gerir informações do sistema',
      changeexternalcanister: 'Alterar canister gerenciado',
    },
  },
};
