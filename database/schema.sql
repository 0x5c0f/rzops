



-- Name: set_updated_at(); Type: FUNCTION; Schema: public; Owner: -

CREATE FUNCTION public.set_updated_at() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$;







-- Name: cmdb_attachment; Type: TABLE; Schema: public; Owner: -

CREATE TABLE public.cmdb_attachment (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    filename character varying(255) NOT NULL,
    target_type character varying(50),
    target_id uuid,
    storage_key character varying(500),
    content_type character varying(100),
    size_bytes bigint,
    uploaded_by_id uuid,
    status character varying(30) DEFAULT 'active'::character varying NOT NULL,
    remarks text,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    deleted_at timestamp with time zone
);


-- Name: cmdb_audit_log; Type: TABLE; Schema: public; Owner: -

CREATE TABLE public.cmdb_audit_log (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    actor_id uuid,
    action character varying(100) NOT NULL,
    resource_type character varying(100) NOT NULL,
    resource_id uuid,
    ip_address character varying(45),
    user_agent character varying(500),
    extra_data jsonb DEFAULT '{}'::jsonb NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);


-- Name: cmdb_backup_plan; Type: TABLE; Schema: public; Owner: -

CREATE TABLE public.cmdb_backup_plan (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    name character varying(100) NOT NULL,
    target_type character varying(50),
    target_id uuid,
    schedule character varying(100),
    retention_days integer,
    status character varying(30) DEFAULT 'draft'::character varying NOT NULL,
    remarks text,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    deleted_at timestamp with time zone,
    CONSTRAINT chk_backup_plan_target_type CHECK (((target_type IS NULL) OR ((target_type)::text = ANY ((ARRAY['server'::character varying, 'database'::character varying, 'site'::character varying])::text[]))))
);


-- Name: cmdb_certificate; Type: TABLE; Schema: public; Owner: -

CREATE TABLE public.cmdb_certificate (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    name character varying(100) NOT NULL,
    provider_id uuid,
    lease_start_date date,
    lease_end_date date,
    certificate_type character varying(50),
    status character varying(30) DEFAULT 'active'::character varying NOT NULL,
    remarks text,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    deleted_at timestamp with time zone
);


-- Name: cmdb_certificate_domain; Type: TABLE; Schema: public; Owner: -

CREATE TABLE public.cmdb_certificate_domain (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    certificate_id uuid NOT NULL,
    domain_id uuid,
    domain_pattern character varying(255) NOT NULL,
    is_primary boolean DEFAULT false NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);


-- Name: cmdb_change_record; Type: TABLE; Schema: public; Owner: -

CREATE TABLE public.cmdb_change_record (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    actor_id uuid,
    change_type character varying(50) NOT NULL,
    resource_type character varying(100) NOT NULL,
    resource_id uuid,
    before_data jsonb DEFAULT '{}'::jsonb NOT NULL,
    after_data jsonb DEFAULT '{}'::jsonb NOT NULL,
    remarks text,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);


-- Name: cmdb_contract; Type: TABLE; Schema: public; Owner: -

CREATE TABLE public.cmdb_contract (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    name character varying(100) NOT NULL,
    provider_id uuid,
    subject_type character varying(50),
    subject_id uuid,
    contract_no character varying(100),
    start_date date,
    end_date date,
    amount numeric(12,2),
    currency character varying(3) DEFAULT 'CNY'::character varying NOT NULL,
    status character varying(30) DEFAULT 'draft'::character varying NOT NULL,
    remarks text,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    deleted_at timestamp with time zone
);


-- Name: cmdb_data_center; Type: TABLE; Schema: public; Owner: -

CREATE TABLE public.cmdb_data_center (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    name character varying(100) NOT NULL,
    provider_id uuid,
    phone character varying(50),
    address character varying(500),
    country character varying(50),
    line_type jsonb DEFAULT '[]'::jsonb,
    description text,
    status character varying(30) DEFAULT 'active'::character varying NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    deleted_at timestamp with time zone
);


-- Name: cmdb_database_instance; Type: TABLE; Schema: public; Owner: -

CREATE TABLE public.cmdb_database_instance (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    server_id uuid,
    name character varying(100) NOT NULL,
    db_type character varying(50) NOT NULL,
    description text,
    status character varying(30) DEFAULT 'active'::character varying NOT NULL,
    offline_time timestamp with time zone,
    is_self_installed boolean DEFAULT false NOT NULL,
    importance character varying(50),
    is_ops_managed boolean DEFAULT true NOT NULL,
    port integer,
    instance_name character varying(100),
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    deleted_at timestamp with time zone,
    environment character varying(50),
    CONSTRAINT cmdb_database_instance_port_check CHECK (((port >= 1) AND (port <= 65535)))
);


-- Name: COLUMN cmdb_database_instance.environment; Type: COMMENT; Schema: public; Owner: -

COMMENT ON COLUMN public.cmdb_database_instance.environment IS '环境：prod生产/test测试/staging预发布/dev开发';


-- Name: cmdb_dict; Type: TABLE; Schema: public; Owner: -

CREATE TABLE public.cmdb_dict (
    id uuid NOT NULL,
    dict_type character varying(100) NOT NULL,
    dict_code character varying(100) NOT NULL,
    dict_label character varying(200) NOT NULL,
    sort_order integer DEFAULT 0 NOT NULL,
    enabled boolean DEFAULT true NOT NULL,
    remark character varying(500),
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    extra_data jsonb,
    deleted_at timestamp with time zone
);


-- Name: COLUMN cmdb_dict.extra_data; Type: COMMENT; Schema: public; Owner: -

COMMENT ON COLUMN public.cmdb_dict.extra_data IS '扩展属性（JSON），如 {"color": "green"} 用于状态徽章颜色等';


-- Name: cmdb_domain; Type: TABLE; Schema: public; Owner: -

CREATE TABLE public.cmdb_domain (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    domain_name character varying(255) NOT NULL,
    expiry_date date,
    renewal_amount numeric(12,2),
    renewal_currency character varying(3) DEFAULT 'CNY'::character varying NOT NULL,
    provider_id uuid,
    platform_phone character varying(50),
    domain_email character varying(255),
    privacy_status character varying(50),
    is_enabled boolean DEFAULT true NOT NULL,
    remarks text,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    registered_date date,
    deleted_at timestamp with time zone
);


-- Name: cmdb_monitor_target; Type: TABLE; Schema: public; Owner: -

CREATE TABLE public.cmdb_monitor_target (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    name character varying(100) NOT NULL,
    target_type character varying(50),
    target_id uuid,
    monitor_type character varying(50),
    endpoint character varying(500),
    interval_seconds integer,
    status character varying(30) DEFAULT 'draft'::character varying NOT NULL,
    remarks text,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    deleted_at timestamp with time zone,
    CONSTRAINT chk_monitor_target_target_type CHECK (((target_type IS NULL) OR ((target_type)::text = ANY ((ARRAY['server'::character varying, 'database'::character varying, 'site'::character varying])::text[]))))
);


-- Name: cmdb_ops_site; Type: TABLE; Schema: public; Owner: -

CREATE TABLE public.cmdb_ops_site (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    name character varying(100) NOT NULL,
    url character varying(500),
    service_target character varying(100),
    importance character varying(50),
    online_time timestamp with time zone,
    code_repo_type character varying(50),
    code_repo_url character varying(500),
    purpose text,
    language_runtime character varying(255),
    web_framework character varying(100),
    is_test_site boolean DEFAULT false NOT NULL,
    last_backup_time timestamp with time zone,
    status character varying(30) DEFAULT 'active'::character varying NOT NULL,
    offline_time timestamp with time zone,
    offline_reason character varying(500),
    function_summary text,
    remarks text,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    deleted_at timestamp with time zone,
    environment character varying(50)
);


-- Name: COLUMN cmdb_ops_site.environment; Type: COMMENT; Schema: public; Owner: -

COMMENT ON COLUMN public.cmdb_ops_site.environment IS '环境：prod生产/test测试/staging预发布/dev开发';


-- Name: cmdb_ops_site_database; Type: TABLE; Schema: public; Owner: -

CREATE TABLE public.cmdb_ops_site_database (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    site_id uuid NOT NULL,
    database_instance_id uuid NOT NULL,
    usage_type character varying(50),
    created_at timestamp with time zone DEFAULT now() NOT NULL
);


-- Name: cmdb_ops_site_domain; Type: TABLE; Schema: public; Owner: -

CREATE TABLE public.cmdb_ops_site_domain (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    site_id uuid NOT NULL,
    domain_id uuid NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    domain_role character varying(50)
);


-- Name: COLUMN cmdb_ops_site_domain.domain_role; Type: COMMENT; Schema: public; Owner: -

COMMENT ON COLUMN public.cmdb_ops_site_domain.domain_role IS '域名角色：primary主域名/alias别名/redirect跳转';


-- Name: cmdb_ops_site_server; Type: TABLE; Schema: public; Owner: -

CREATE TABLE public.cmdb_ops_site_server (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    site_id uuid NOT NULL,
    server_id uuid NOT NULL,
    deploy_role character varying(50),
    created_at timestamp with time zone DEFAULT now() NOT NULL
);


-- Name: cmdb_provider; Type: TABLE; Schema: public; Owner: -

CREATE TABLE public.cmdb_provider (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    name character varying(100) NOT NULL,
    provider_types jsonb DEFAULT '[]'::jsonb NOT NULL,
    contact_name character varying(100),
    contact_phone character varying(50),
    contact_qq character varying(50),
    fax character varying(50),
    address character varying(500),
    website character varying(500),
    description text,
    status character varying(30) DEFAULT 'active'::character varying NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    deleted_at timestamp with time zone
);


-- Name: cmdb_server; Type: TABLE; Schema: public; Owner: -

CREATE TABLE public.cmdb_server (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    asset_code character varying(100),
    name character varying(100) NOT NULL,
    primary_ip character varying(45),
    location character varying(255),
    isp_provider_id uuid,
    data_center_id uuid,
    hosting_type character varying(50),
    is_dual_line boolean DEFAULT false NOT NULL,
    lease_start_date date,
    lease_end_date date,
    price numeric(12,2),
    price_currency character varying(3) DEFAULT 'CNY'::character varying NOT NULL,
    server_type character varying(50),
    role_tags jsonb DEFAULT '[]'::jsonb NOT NULL,
    is_database_server boolean DEFAULT false NOT NULL,
    cpu character varying(255),
    memory_gb integer,
    is_raid boolean DEFAULT false NOT NULL,
    raid_level character varying(50),
    disk_layout text,
    hardware_config text,
    architecture character varying(20),
    maintainer_id uuid,
    brand character varying(100),
    warranty_info text,
    operating_system character varying(100),
    web_server_type jsonb DEFAULT '[]'::jsonb NOT NULL,
    server_provider_id uuid,
    software_provider_id uuid,
    status character varying(30) DEFAULT 'active'::character varying NOT NULL,
    offline_time timestamp with time zone,
    offline_reason text,
    remarks text,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    deleted_at timestamp with time zone,
    environment character varying(50)
);


-- Name: COLUMN cmdb_server.environment; Type: COMMENT; Schema: public; Owner: -

COMMENT ON COLUMN public.cmdb_server.environment IS '环境：prod生产/test测试/staging预发布/dev开发';


-- Name: cmdb_server_ip; Type: TABLE; Schema: public; Owner: -

CREATE TABLE public.cmdb_server_ip (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    server_id uuid,
    ip_address character varying(45) NOT NULL,
    ip_type character varying(50) DEFAULT 'public'::character varying NOT NULL,
    is_primary boolean DEFAULT false NOT NULL,
    isp_provider_id uuid,
    description text,
    status character varying(30) DEFAULT 'enabled'::character varying NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    nic_name character varying(100),
    deleted_at timestamp with time zone
);


-- Name: COLUMN cmdb_server_ip.nic_name; Type: COMMENT; Schema: public; Owner: -

COMMENT ON COLUMN public.cmdb_server_ip.nic_name IS '网卡名称，如 eth0 / ens33 / 内网网卡';


-- Name: cmdb_server_port; Type: TABLE; Schema: public; Owner: -

CREATE TABLE public.cmdb_server_port (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    protocol character varying(20) NOT NULL,
    port integer NOT NULL,
    service_name character varying(100) NOT NULL,
    access_scope character varying(50),
    is_enabled boolean DEFAULT true NOT NULL,
    description text,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    server_id uuid NOT NULL,
    deleted_at timestamp with time zone,
    CONSTRAINT cmdb_server_port_port_check CHECK (((port >= 1) AND (port <= 65535)))
);


-- Name: cmdb_server_port_template; Type: TABLE; Schema: public; Owner: -

CREATE TABLE public.cmdb_server_port_template (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    name character varying(100) NOT NULL,
    protocol character varying(20) NOT NULL,
    port integer NOT NULL,
    service_name character varying(100) NOT NULL,
    access_scope character varying(50),
    is_enabled boolean DEFAULT true NOT NULL,
    description text,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    deleted_at timestamp with time zone
);


-- Name: role; Type: TABLE; Schema: public; Owner: -

CREATE TABLE public.role (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    code character varying(50) NOT NULL,
    name character varying(100) NOT NULL,
    description text,
    is_builtin boolean DEFAULT false NOT NULL,
    is_active boolean DEFAULT true NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    deleted_at timestamp with time zone
);


-- Name: TABLE role; Type: COMMENT; Schema: public; Owner: -

COMMENT ON TABLE public.role IS '角色表';


-- Name: COLUMN role.deleted_at; Type: COMMENT; Schema: public; Owner: -

COMMENT ON COLUMN public.role.deleted_at IS '软删除时间，非空表示已删除';


-- Name: role_permission; Type: TABLE; Schema: public; Owner: -

CREATE TABLE public.role_permission (
    role_id uuid NOT NULL,
    permission_code character varying(100) NOT NULL
);


-- Name: TABLE role_permission; Type: COMMENT; Schema: public; Owner: -

COMMENT ON TABLE public.role_permission IS '角色-权限关联表';


-- Name: user; Type: TABLE; Schema: public; Owner: -

CREATE TABLE public."user" (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    email character varying(255) NOT NULL,
    hashed_password character varying(255) NOT NULL,
    is_active boolean DEFAULT true NOT NULL,
    is_superuser boolean DEFAULT false NOT NULL,
    full_name character varying(255),
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    deleted_at timestamp with time zone
);


-- Name: COLUMN "user".deleted_at; Type: COMMENT; Schema: public; Owner: -

COMMENT ON COLUMN public."user".deleted_at IS '软删除时间，非空表示已删除';


-- Name: user_role; Type: TABLE; Schema: public; Owner: -

CREATE TABLE public.user_role (
    user_id uuid NOT NULL,
    role_id uuid NOT NULL
);


-- Name: TABLE user_role; Type: COMMENT; Schema: public; Owner: -

COMMENT ON TABLE public.user_role IS '用户-角色关联表（多对多）';





-- Name: cmdb_attachment cmdb_attachment_pkey; Type: CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.cmdb_attachment
    ADD CONSTRAINT cmdb_attachment_pkey PRIMARY KEY (id);


-- Name: cmdb_audit_log cmdb_audit_log_pkey; Type: CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.cmdb_audit_log
    ADD CONSTRAINT cmdb_audit_log_pkey PRIMARY KEY (id);


-- Name: cmdb_backup_plan cmdb_backup_plan_pkey; Type: CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.cmdb_backup_plan
    ADD CONSTRAINT cmdb_backup_plan_pkey PRIMARY KEY (id);


-- Name: cmdb_certificate_domain cmdb_certificate_domain_pkey; Type: CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.cmdb_certificate_domain
    ADD CONSTRAINT cmdb_certificate_domain_pkey PRIMARY KEY (id);


-- Name: cmdb_certificate cmdb_certificate_pkey; Type: CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.cmdb_certificate
    ADD CONSTRAINT cmdb_certificate_pkey PRIMARY KEY (id);


-- Name: cmdb_change_record cmdb_change_record_pkey; Type: CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.cmdb_change_record
    ADD CONSTRAINT cmdb_change_record_pkey PRIMARY KEY (id);


-- Name: cmdb_contract cmdb_contract_pkey; Type: CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.cmdb_contract
    ADD CONSTRAINT cmdb_contract_pkey PRIMARY KEY (id);


-- Name: cmdb_data_center cmdb_data_center_pkey; Type: CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.cmdb_data_center
    ADD CONSTRAINT cmdb_data_center_pkey PRIMARY KEY (id);


-- Name: cmdb_database_instance cmdb_database_instance_pkey; Type: CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.cmdb_database_instance
    ADD CONSTRAINT cmdb_database_instance_pkey PRIMARY KEY (id);


-- Name: cmdb_dict cmdb_dict_pkey; Type: CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.cmdb_dict
    ADD CONSTRAINT cmdb_dict_pkey PRIMARY KEY (id);


-- Name: cmdb_domain cmdb_domain_domain_name_key; Type: CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.cmdb_domain
    ADD CONSTRAINT cmdb_domain_domain_name_key UNIQUE (domain_name);


-- Name: cmdb_domain cmdb_domain_pkey; Type: CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.cmdb_domain
    ADD CONSTRAINT cmdb_domain_pkey PRIMARY KEY (id);


-- Name: cmdb_monitor_target cmdb_monitor_target_pkey; Type: CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.cmdb_monitor_target
    ADD CONSTRAINT cmdb_monitor_target_pkey PRIMARY KEY (id);


-- Name: cmdb_ops_site_database cmdb_ops_site_database_pkey; Type: CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.cmdb_ops_site_database
    ADD CONSTRAINT cmdb_ops_site_database_pkey PRIMARY KEY (id);


-- Name: cmdb_ops_site_domain cmdb_ops_site_domain_pkey; Type: CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.cmdb_ops_site_domain
    ADD CONSTRAINT cmdb_ops_site_domain_pkey PRIMARY KEY (id);


-- Name: cmdb_ops_site cmdb_ops_site_pkey; Type: CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.cmdb_ops_site
    ADD CONSTRAINT cmdb_ops_site_pkey PRIMARY KEY (id);


-- Name: cmdb_ops_site_server cmdb_ops_site_server_pkey; Type: CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.cmdb_ops_site_server
    ADD CONSTRAINT cmdb_ops_site_server_pkey PRIMARY KEY (id);


-- Name: cmdb_provider cmdb_provider_name_key; Type: CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.cmdb_provider
    ADD CONSTRAINT cmdb_provider_name_key UNIQUE (name);


-- Name: cmdb_provider cmdb_provider_pkey; Type: CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.cmdb_provider
    ADD CONSTRAINT cmdb_provider_pkey PRIMARY KEY (id);


-- Name: cmdb_server cmdb_server_asset_code_key; Type: CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.cmdb_server
    ADD CONSTRAINT cmdb_server_asset_code_key UNIQUE (asset_code);


-- Name: cmdb_server_ip cmdb_server_ip_ip_address_key; Type: CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.cmdb_server_ip
    ADD CONSTRAINT cmdb_server_ip_ip_address_key UNIQUE (ip_address);


-- Name: cmdb_server_ip cmdb_server_ip_pkey; Type: CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.cmdb_server_ip
    ADD CONSTRAINT cmdb_server_ip_pkey PRIMARY KEY (id);


-- Name: cmdb_server cmdb_server_pkey; Type: CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.cmdb_server
    ADD CONSTRAINT cmdb_server_pkey PRIMARY KEY (id);


-- Name: cmdb_server_port cmdb_server_port_pkey; Type: CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.cmdb_server_port
    ADD CONSTRAINT cmdb_server_port_pkey PRIMARY KEY (id);


-- Name: cmdb_server_port_template cmdb_server_port_template_pkey; Type: CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.cmdb_server_port_template
    ADD CONSTRAINT cmdb_server_port_template_pkey PRIMARY KEY (id);


-- Name: role role_code_key; Type: CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.role
    ADD CONSTRAINT role_code_key UNIQUE (code);


-- Name: role_permission role_permission_pkey; Type: CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.role_permission
    ADD CONSTRAINT role_permission_pkey PRIMARY KEY (role_id, permission_code);


-- Name: role role_pkey; Type: CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.role
    ADD CONSTRAINT role_pkey PRIMARY KEY (id);


-- Name: cmdb_certificate_domain uq_cmdb_certificate_domain_certificate_pattern; Type: CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.cmdb_certificate_domain
    ADD CONSTRAINT uq_cmdb_certificate_domain_certificate_pattern UNIQUE (certificate_id, domain_pattern);


-- Name: cmdb_data_center uq_cmdb_data_center_provider_name; Type: CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.cmdb_data_center
    ADD CONSTRAINT uq_cmdb_data_center_provider_name UNIQUE (provider_id, name);


-- Name: cmdb_database_instance uq_cmdb_database_instance_server_type_port_instance; Type: CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.cmdb_database_instance
    ADD CONSTRAINT uq_cmdb_database_instance_server_type_port_instance UNIQUE (server_id, db_type, port, instance_name);


-- Name: cmdb_dict uq_cmdb_dict_type_code; Type: CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.cmdb_dict
    ADD CONSTRAINT uq_cmdb_dict_type_code UNIQUE (dict_type, dict_code);


-- Name: cmdb_ops_site_database uq_cmdb_ops_site_database_site_database_usage; Type: CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.cmdb_ops_site_database
    ADD CONSTRAINT uq_cmdb_ops_site_database_site_database_usage UNIQUE (site_id, database_instance_id, usage_type);


-- Name: cmdb_ops_site_domain uq_cmdb_ops_site_domain_site_domain; Type: CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.cmdb_ops_site_domain
    ADD CONSTRAINT uq_cmdb_ops_site_domain_site_domain UNIQUE (site_id, domain_id);


-- Name: cmdb_ops_site_server uq_cmdb_ops_site_server_site_server_role; Type: CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.cmdb_ops_site_server
    ADD CONSTRAINT uq_cmdb_ops_site_server_site_server_role UNIQUE (site_id, server_id, deploy_role);


-- Name: cmdb_server_port uq_cmdb_server_port_server_proto_port; Type: CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.cmdb_server_port
    ADD CONSTRAINT uq_cmdb_server_port_server_proto_port UNIQUE (server_id, protocol, port);


-- Name: user user_email_key; Type: CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public."user"
    ADD CONSTRAINT user_email_key UNIQUE (email);


-- Name: user user_pkey; Type: CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public."user"
    ADD CONSTRAINT user_pkey PRIMARY KEY (id);


-- Name: user_role user_role_pkey; Type: CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.user_role
    ADD CONSTRAINT user_role_pkey PRIMARY KEY (user_id, role_id);


-- Name: idx_cmdb_server_port_server_id; Type: INDEX; Schema: public; Owner: -

CREATE INDEX idx_cmdb_server_port_server_id ON public.cmdb_server_port USING btree (server_id);


-- Name: ix_cmdb_attachment_filename; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_attachment_filename ON public.cmdb_attachment USING btree (filename);


-- Name: ix_cmdb_attachment_status; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_attachment_status ON public.cmdb_attachment USING btree (status);


-- Name: ix_cmdb_attachment_target_id; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_attachment_target_id ON public.cmdb_attachment USING btree (target_id);


-- Name: ix_cmdb_attachment_target_type; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_attachment_target_type ON public.cmdb_attachment USING btree (target_type);


-- Name: ix_cmdb_attachment_uploaded_by_id; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_attachment_uploaded_by_id ON public.cmdb_attachment USING btree (uploaded_by_id);


-- Name: ix_cmdb_audit_log_action; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_audit_log_action ON public.cmdb_audit_log USING btree (action);


-- Name: ix_cmdb_audit_log_actor_id; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_audit_log_actor_id ON public.cmdb_audit_log USING btree (actor_id);


-- Name: ix_cmdb_audit_log_created_at; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_audit_log_created_at ON public.cmdb_audit_log USING btree (created_at);


-- Name: ix_cmdb_audit_log_resource_id; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_audit_log_resource_id ON public.cmdb_audit_log USING btree (resource_id);


-- Name: ix_cmdb_audit_log_resource_type; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_audit_log_resource_type ON public.cmdb_audit_log USING btree (resource_type);


-- Name: ix_cmdb_backup_plan_name; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_backup_plan_name ON public.cmdb_backup_plan USING btree (name);


-- Name: ix_cmdb_backup_plan_status; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_backup_plan_status ON public.cmdb_backup_plan USING btree (status);


-- Name: ix_cmdb_backup_plan_target_id; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_backup_plan_target_id ON public.cmdb_backup_plan USING btree (target_id);


-- Name: ix_cmdb_backup_plan_target_type; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_backup_plan_target_type ON public.cmdb_backup_plan USING btree (target_type);


-- Name: ix_cmdb_certificate_alive; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_certificate_alive ON public.cmdb_certificate USING btree (deleted_at) WHERE (deleted_at IS NULL);


-- Name: ix_cmdb_certificate_certificate_type; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_certificate_certificate_type ON public.cmdb_certificate USING btree (certificate_type);


-- Name: ix_cmdb_certificate_domain_certificate_id; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_certificate_domain_certificate_id ON public.cmdb_certificate_domain USING btree (certificate_id);


-- Name: ix_cmdb_certificate_domain_domain_id; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_certificate_domain_domain_id ON public.cmdb_certificate_domain USING btree (domain_id);


-- Name: ix_cmdb_certificate_domain_domain_pattern; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_certificate_domain_domain_pattern ON public.cmdb_certificate_domain USING btree (domain_pattern);


-- Name: ix_cmdb_certificate_domain_is_primary; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_certificate_domain_is_primary ON public.cmdb_certificate_domain USING btree (is_primary);


-- Name: ix_cmdb_certificate_lease_end_date; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_certificate_lease_end_date ON public.cmdb_certificate USING btree (lease_end_date);


-- Name: ix_cmdb_certificate_name; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_certificate_name ON public.cmdb_certificate USING btree (name);


-- Name: ix_cmdb_certificate_provider_id; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_certificate_provider_id ON public.cmdb_certificate USING btree (provider_id);


-- Name: ix_cmdb_certificate_status; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_certificate_status ON public.cmdb_certificate USING btree (status);


-- Name: ix_cmdb_change_record_actor_id; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_change_record_actor_id ON public.cmdb_change_record USING btree (actor_id);


-- Name: ix_cmdb_change_record_change_type; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_change_record_change_type ON public.cmdb_change_record USING btree (change_type);


-- Name: ix_cmdb_change_record_created_at; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_change_record_created_at ON public.cmdb_change_record USING btree (created_at);


-- Name: ix_cmdb_change_record_resource_id; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_change_record_resource_id ON public.cmdb_change_record USING btree (resource_id);


-- Name: ix_cmdb_change_record_resource_type; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_change_record_resource_type ON public.cmdb_change_record USING btree (resource_type);


-- Name: ix_cmdb_contract_contract_no; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_contract_contract_no ON public.cmdb_contract USING btree (contract_no);


-- Name: ix_cmdb_contract_end_date; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_contract_end_date ON public.cmdb_contract USING btree (end_date);


-- Name: ix_cmdb_contract_name; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_contract_name ON public.cmdb_contract USING btree (name);


-- Name: ix_cmdb_contract_provider_id; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_contract_provider_id ON public.cmdb_contract USING btree (provider_id);


-- Name: ix_cmdb_contract_status; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_contract_status ON public.cmdb_contract USING btree (status);


-- Name: ix_cmdb_contract_subject_id; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_contract_subject_id ON public.cmdb_contract USING btree (subject_id);


-- Name: ix_cmdb_contract_subject_type; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_contract_subject_type ON public.cmdb_contract USING btree (subject_type);


-- Name: ix_cmdb_data_center_country; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_data_center_country ON public.cmdb_data_center USING btree (country);


-- Name: ix_cmdb_data_center_line_type; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_data_center_line_type ON public.cmdb_data_center USING btree (line_type);


-- Name: ix_cmdb_data_center_name; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_data_center_name ON public.cmdb_data_center USING btree (name);


-- Name: ix_cmdb_data_center_provider_id; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_data_center_provider_id ON public.cmdb_data_center USING btree (provider_id);


-- Name: ix_cmdb_data_center_status; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_data_center_status ON public.cmdb_data_center USING btree (status);


-- Name: ix_cmdb_database_instance_alive; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_database_instance_alive ON public.cmdb_database_instance USING btree (deleted_at) WHERE (deleted_at IS NULL);


-- Name: ix_cmdb_database_instance_db_type; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_database_instance_db_type ON public.cmdb_database_instance USING btree (db_type);


-- Name: ix_cmdb_database_instance_importance; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_database_instance_importance ON public.cmdb_database_instance USING btree (importance);


-- Name: ix_cmdb_database_instance_instance_name; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_database_instance_instance_name ON public.cmdb_database_instance USING btree (instance_name);


-- Name: ix_cmdb_database_instance_is_ops_managed; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_database_instance_is_ops_managed ON public.cmdb_database_instance USING btree (is_ops_managed);


-- Name: ix_cmdb_database_instance_is_self_installed; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_database_instance_is_self_installed ON public.cmdb_database_instance USING btree (is_self_installed);


-- Name: ix_cmdb_database_instance_name; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_database_instance_name ON public.cmdb_database_instance USING btree (name);


-- Name: ix_cmdb_database_instance_offline_time; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_database_instance_offline_time ON public.cmdb_database_instance USING btree (offline_time);


-- Name: ix_cmdb_database_instance_port; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_database_instance_port ON public.cmdb_database_instance USING btree (port);


-- Name: ix_cmdb_database_instance_server_id; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_database_instance_server_id ON public.cmdb_database_instance USING btree (server_id);


-- Name: ix_cmdb_database_instance_status; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_database_instance_status ON public.cmdb_database_instance USING btree (status);


-- Name: ix_cmdb_dict_type; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_dict_type ON public.cmdb_dict USING btree (dict_type);


-- Name: ix_cmdb_domain_alive; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_domain_alive ON public.cmdb_domain USING btree (deleted_at) WHERE (deleted_at IS NULL);


-- Name: ix_cmdb_domain_expiry_date; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_domain_expiry_date ON public.cmdb_domain USING btree (expiry_date);


-- Name: ix_cmdb_domain_is_enabled; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_domain_is_enabled ON public.cmdb_domain USING btree (is_enabled);


-- Name: ix_cmdb_domain_privacy_status; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_domain_privacy_status ON public.cmdb_domain USING btree (privacy_status);


-- Name: ix_cmdb_domain_provider_id; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_domain_provider_id ON public.cmdb_domain USING btree (provider_id);


-- Name: ix_cmdb_monitor_target_monitor_type; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_monitor_target_monitor_type ON public.cmdb_monitor_target USING btree (monitor_type);


-- Name: ix_cmdb_monitor_target_name; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_monitor_target_name ON public.cmdb_monitor_target USING btree (name);


-- Name: ix_cmdb_monitor_target_status; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_monitor_target_status ON public.cmdb_monitor_target USING btree (status);


-- Name: ix_cmdb_monitor_target_target_id; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_monitor_target_target_id ON public.cmdb_monitor_target USING btree (target_id);


-- Name: ix_cmdb_monitor_target_target_type; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_monitor_target_target_type ON public.cmdb_monitor_target USING btree (target_type);


-- Name: ix_cmdb_ops_site_alive; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_ops_site_alive ON public.cmdb_ops_site USING btree (deleted_at) WHERE (deleted_at IS NULL);


-- Name: ix_cmdb_ops_site_database_database_instance_id; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_ops_site_database_database_instance_id ON public.cmdb_ops_site_database USING btree (database_instance_id);


-- Name: ix_cmdb_ops_site_database_site_id; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_ops_site_database_site_id ON public.cmdb_ops_site_database USING btree (site_id);


-- Name: ix_cmdb_ops_site_database_usage_type; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_ops_site_database_usage_type ON public.cmdb_ops_site_database USING btree (usage_type);


-- Name: ix_cmdb_ops_site_domain_domain_id; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_ops_site_domain_domain_id ON public.cmdb_ops_site_domain USING btree (domain_id);


-- Name: ix_cmdb_ops_site_domain_site_id; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_ops_site_domain_site_id ON public.cmdb_ops_site_domain USING btree (site_id);


-- Name: ix_cmdb_ops_site_importance; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_ops_site_importance ON public.cmdb_ops_site USING btree (importance);


-- Name: ix_cmdb_ops_site_is_test_site; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_ops_site_is_test_site ON public.cmdb_ops_site USING btree (is_test_site);


-- Name: ix_cmdb_ops_site_last_backup_time; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_ops_site_last_backup_time ON public.cmdb_ops_site USING btree (last_backup_time);


-- Name: ix_cmdb_ops_site_name; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_ops_site_name ON public.cmdb_ops_site USING btree (name);


-- Name: ix_cmdb_ops_site_offline_time; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_ops_site_offline_time ON public.cmdb_ops_site USING btree (offline_time);


-- Name: ix_cmdb_ops_site_online_time; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_ops_site_online_time ON public.cmdb_ops_site USING btree (online_time);


-- Name: ix_cmdb_ops_site_server_deploy_role; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_ops_site_server_deploy_role ON public.cmdb_ops_site_server USING btree (deploy_role);


-- Name: ix_cmdb_ops_site_server_server_id; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_ops_site_server_server_id ON public.cmdb_ops_site_server USING btree (server_id);


-- Name: ix_cmdb_ops_site_server_site_id; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_ops_site_server_site_id ON public.cmdb_ops_site_server USING btree (site_id);


-- Name: ix_cmdb_ops_site_service_target; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_ops_site_service_target ON public.cmdb_ops_site USING btree (service_target);


-- Name: ix_cmdb_ops_site_status; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_ops_site_status ON public.cmdb_ops_site USING btree (status);


-- Name: ix_cmdb_ops_site_url; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_ops_site_url ON public.cmdb_ops_site USING btree (url);


-- Name: ix_cmdb_ops_site_web_framework; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_ops_site_web_framework ON public.cmdb_ops_site USING btree (web_framework);


-- Name: ix_cmdb_provider_status; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_provider_status ON public.cmdb_provider USING btree (status);


-- Name: ix_cmdb_server_alive; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_server_alive ON public.cmdb_server USING btree (deleted_at) WHERE (deleted_at IS NULL);


-- Name: ix_cmdb_server_data_center_id; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_server_data_center_id ON public.cmdb_server USING btree (data_center_id);


-- Name: ix_cmdb_server_hosting_type; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_server_hosting_type ON public.cmdb_server USING btree (hosting_type);


-- Name: ix_cmdb_server_ip_ip_type; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_server_ip_ip_type ON public.cmdb_server_ip USING btree (ip_type);


-- Name: ix_cmdb_server_ip_is_primary; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_server_ip_is_primary ON public.cmdb_server_ip USING btree (is_primary);


-- Name: ix_cmdb_server_ip_isp_provider_id; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_server_ip_isp_provider_id ON public.cmdb_server_ip USING btree (isp_provider_id);


-- Name: ix_cmdb_server_ip_server_id; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_server_ip_server_id ON public.cmdb_server_ip USING btree (server_id);


-- Name: ix_cmdb_server_ip_status; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_server_ip_status ON public.cmdb_server_ip USING btree (status);


-- Name: ix_cmdb_server_is_database_server; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_server_is_database_server ON public.cmdb_server USING btree (is_database_server);


-- Name: ix_cmdb_server_isp_provider_id; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_server_isp_provider_id ON public.cmdb_server USING btree (isp_provider_id);


-- Name: ix_cmdb_server_lease_end_date; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_server_lease_end_date ON public.cmdb_server USING btree (lease_end_date);


-- Name: ix_cmdb_server_maintainer_id; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_server_maintainer_id ON public.cmdb_server USING btree (maintainer_id);


-- Name: ix_cmdb_server_name; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_server_name ON public.cmdb_server USING btree (name);


-- Name: ix_cmdb_server_offline_time; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_server_offline_time ON public.cmdb_server USING btree (offline_time);


-- Name: ix_cmdb_server_operating_system; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_server_operating_system ON public.cmdb_server USING btree (operating_system);


-- Name: ix_cmdb_server_port_access_scope; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_server_port_access_scope ON public.cmdb_server_port USING btree (access_scope);


-- Name: ix_cmdb_server_port_is_enabled; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_server_port_is_enabled ON public.cmdb_server_port USING btree (is_enabled);


-- Name: ix_cmdb_server_port_port; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_server_port_port ON public.cmdb_server_port USING btree (port);


-- Name: ix_cmdb_server_port_protocol; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_server_port_protocol ON public.cmdb_server_port USING btree (protocol);


-- Name: ix_cmdb_server_port_service_name; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_server_port_service_name ON public.cmdb_server_port USING btree (service_name);


-- Name: ix_cmdb_server_primary_ip; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_server_primary_ip ON public.cmdb_server USING btree (primary_ip);


-- Name: ix_cmdb_server_server_provider_id; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_server_server_provider_id ON public.cmdb_server USING btree (server_provider_id);


-- Name: ix_cmdb_server_server_type; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_server_server_type ON public.cmdb_server USING btree (server_type);


-- Name: ix_cmdb_server_software_provider_id; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_server_software_provider_id ON public.cmdb_server USING btree (software_provider_id);


-- Name: ix_cmdb_server_status; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_cmdb_server_status ON public.cmdb_server USING btree (status);


-- Name: ix_role_permission_code; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_role_permission_code ON public.role_permission USING btree (permission_code);


-- Name: ix_user_email; Type: INDEX; Schema: public; Owner: -

CREATE INDEX ix_user_email ON public."user" USING btree (email);


-- Name: cmdb_attachment trg_cmdb_attachment_updated_at; Type: TRIGGER; Schema: public; Owner: -

CREATE TRIGGER trg_cmdb_attachment_updated_at BEFORE UPDATE ON public.cmdb_attachment FOR EACH ROW EXECUTE FUNCTION public.set_updated_at();


-- Name: cmdb_backup_plan trg_cmdb_backup_plan_updated_at; Type: TRIGGER; Schema: public; Owner: -

CREATE TRIGGER trg_cmdb_backup_plan_updated_at BEFORE UPDATE ON public.cmdb_backup_plan FOR EACH ROW EXECUTE FUNCTION public.set_updated_at();


-- Name: cmdb_certificate trg_cmdb_certificate_updated_at; Type: TRIGGER; Schema: public; Owner: -

CREATE TRIGGER trg_cmdb_certificate_updated_at BEFORE UPDATE ON public.cmdb_certificate FOR EACH ROW EXECUTE FUNCTION public.set_updated_at();


-- Name: cmdb_contract trg_cmdb_contract_updated_at; Type: TRIGGER; Schema: public; Owner: -

CREATE TRIGGER trg_cmdb_contract_updated_at BEFORE UPDATE ON public.cmdb_contract FOR EACH ROW EXECUTE FUNCTION public.set_updated_at();


-- Name: cmdb_data_center trg_cmdb_data_center_updated_at; Type: TRIGGER; Schema: public; Owner: -

CREATE TRIGGER trg_cmdb_data_center_updated_at BEFORE UPDATE ON public.cmdb_data_center FOR EACH ROW EXECUTE FUNCTION public.set_updated_at();


-- Name: cmdb_database_instance trg_cmdb_database_instance_updated_at; Type: TRIGGER; Schema: public; Owner: -

CREATE TRIGGER trg_cmdb_database_instance_updated_at BEFORE UPDATE ON public.cmdb_database_instance FOR EACH ROW EXECUTE FUNCTION public.set_updated_at();


-- Name: cmdb_dict trg_cmdb_dict_updated_at; Type: TRIGGER; Schema: public; Owner: -

CREATE TRIGGER trg_cmdb_dict_updated_at BEFORE UPDATE ON public.cmdb_dict FOR EACH ROW EXECUTE FUNCTION public.set_updated_at();


-- Name: cmdb_domain trg_cmdb_domain_updated_at; Type: TRIGGER; Schema: public; Owner: -

CREATE TRIGGER trg_cmdb_domain_updated_at BEFORE UPDATE ON public.cmdb_domain FOR EACH ROW EXECUTE FUNCTION public.set_updated_at();


-- Name: cmdb_monitor_target trg_cmdb_monitor_target_updated_at; Type: TRIGGER; Schema: public; Owner: -

CREATE TRIGGER trg_cmdb_monitor_target_updated_at BEFORE UPDATE ON public.cmdb_monitor_target FOR EACH ROW EXECUTE FUNCTION public.set_updated_at();


-- Name: cmdb_ops_site trg_cmdb_ops_site_updated_at; Type: TRIGGER; Schema: public; Owner: -

CREATE TRIGGER trg_cmdb_ops_site_updated_at BEFORE UPDATE ON public.cmdb_ops_site FOR EACH ROW EXECUTE FUNCTION public.set_updated_at();


-- Name: cmdb_provider trg_cmdb_provider_updated_at; Type: TRIGGER; Schema: public; Owner: -

CREATE TRIGGER trg_cmdb_provider_updated_at BEFORE UPDATE ON public.cmdb_provider FOR EACH ROW EXECUTE FUNCTION public.set_updated_at();


-- Name: cmdb_server_ip trg_cmdb_server_ip_updated_at; Type: TRIGGER; Schema: public; Owner: -

CREATE TRIGGER trg_cmdb_server_ip_updated_at BEFORE UPDATE ON public.cmdb_server_ip FOR EACH ROW EXECUTE FUNCTION public.set_updated_at();


-- Name: cmdb_server_port_template trg_cmdb_server_port_template_updated_at; Type: TRIGGER; Schema: public; Owner: -

CREATE TRIGGER trg_cmdb_server_port_template_updated_at BEFORE UPDATE ON public.cmdb_server_port_template FOR EACH ROW EXECUTE FUNCTION public.set_updated_at();


-- Name: cmdb_server_port trg_cmdb_server_port_updated_at; Type: TRIGGER; Schema: public; Owner: -

CREATE TRIGGER trg_cmdb_server_port_updated_at BEFORE UPDATE ON public.cmdb_server_port FOR EACH ROW EXECUTE FUNCTION public.set_updated_at();


-- Name: cmdb_server trg_cmdb_server_updated_at; Type: TRIGGER; Schema: public; Owner: -

CREATE TRIGGER trg_cmdb_server_updated_at BEFORE UPDATE ON public.cmdb_server FOR EACH ROW EXECUTE FUNCTION public.set_updated_at();


-- Name: role trg_role_updated_at; Type: TRIGGER; Schema: public; Owner: -

CREATE TRIGGER trg_role_updated_at BEFORE UPDATE ON public.role FOR EACH ROW EXECUTE FUNCTION public.set_updated_at();


-- Name: cmdb_attachment cmdb_attachment_uploaded_by_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.cmdb_attachment
    ADD CONSTRAINT cmdb_attachment_uploaded_by_id_fkey FOREIGN KEY (uploaded_by_id) REFERENCES public."user"(id) ON DELETE SET NULL;


-- Name: cmdb_audit_log cmdb_audit_log_actor_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.cmdb_audit_log
    ADD CONSTRAINT cmdb_audit_log_actor_id_fkey FOREIGN KEY (actor_id) REFERENCES public."user"(id) ON DELETE SET NULL;


-- Name: cmdb_certificate_domain cmdb_certificate_domain_certificate_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.cmdb_certificate_domain
    ADD CONSTRAINT cmdb_certificate_domain_certificate_id_fkey FOREIGN KEY (certificate_id) REFERENCES public.cmdb_certificate(id) ON DELETE CASCADE;


-- Name: cmdb_certificate_domain cmdb_certificate_domain_domain_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.cmdb_certificate_domain
    ADD CONSTRAINT cmdb_certificate_domain_domain_id_fkey FOREIGN KEY (domain_id) REFERENCES public.cmdb_domain(id) ON DELETE CASCADE;


-- Name: cmdb_certificate cmdb_certificate_provider_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.cmdb_certificate
    ADD CONSTRAINT cmdb_certificate_provider_id_fkey FOREIGN KEY (provider_id) REFERENCES public.cmdb_provider(id) ON DELETE SET NULL;


-- Name: cmdb_change_record cmdb_change_record_actor_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.cmdb_change_record
    ADD CONSTRAINT cmdb_change_record_actor_id_fkey FOREIGN KEY (actor_id) REFERENCES public."user"(id) ON DELETE SET NULL;


-- Name: cmdb_contract cmdb_contract_provider_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.cmdb_contract
    ADD CONSTRAINT cmdb_contract_provider_id_fkey FOREIGN KEY (provider_id) REFERENCES public.cmdb_provider(id) ON DELETE SET NULL;


-- Name: cmdb_data_center cmdb_data_center_provider_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.cmdb_data_center
    ADD CONSTRAINT cmdb_data_center_provider_id_fkey FOREIGN KEY (provider_id) REFERENCES public.cmdb_provider(id) ON DELETE SET NULL;


-- Name: cmdb_database_instance cmdb_database_instance_server_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.cmdb_database_instance
    ADD CONSTRAINT cmdb_database_instance_server_id_fkey FOREIGN KEY (server_id) REFERENCES public.cmdb_server(id) ON DELETE SET NULL;


-- Name: cmdb_domain cmdb_domain_provider_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.cmdb_domain
    ADD CONSTRAINT cmdb_domain_provider_id_fkey FOREIGN KEY (provider_id) REFERENCES public.cmdb_provider(id) ON DELETE SET NULL;


-- Name: cmdb_ops_site_database cmdb_ops_site_database_database_instance_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.cmdb_ops_site_database
    ADD CONSTRAINT cmdb_ops_site_database_database_instance_id_fkey FOREIGN KEY (database_instance_id) REFERENCES public.cmdb_database_instance(id) ON DELETE CASCADE;


-- Name: cmdb_ops_site_database cmdb_ops_site_database_site_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.cmdb_ops_site_database
    ADD CONSTRAINT cmdb_ops_site_database_site_id_fkey FOREIGN KEY (site_id) REFERENCES public.cmdb_ops_site(id) ON DELETE CASCADE;


-- Name: cmdb_ops_site_domain cmdb_ops_site_domain_domain_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.cmdb_ops_site_domain
    ADD CONSTRAINT cmdb_ops_site_domain_domain_id_fkey FOREIGN KEY (domain_id) REFERENCES public.cmdb_domain(id) ON DELETE CASCADE;


-- Name: cmdb_ops_site_domain cmdb_ops_site_domain_site_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.cmdb_ops_site_domain
    ADD CONSTRAINT cmdb_ops_site_domain_site_id_fkey FOREIGN KEY (site_id) REFERENCES public.cmdb_ops_site(id) ON DELETE CASCADE;


-- Name: cmdb_ops_site_server cmdb_ops_site_server_server_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.cmdb_ops_site_server
    ADD CONSTRAINT cmdb_ops_site_server_server_id_fkey FOREIGN KEY (server_id) REFERENCES public.cmdb_server(id) ON DELETE CASCADE;


-- Name: cmdb_ops_site_server cmdb_ops_site_server_site_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.cmdb_ops_site_server
    ADD CONSTRAINT cmdb_ops_site_server_site_id_fkey FOREIGN KEY (site_id) REFERENCES public.cmdb_ops_site(id) ON DELETE CASCADE;


-- Name: cmdb_server cmdb_server_data_center_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.cmdb_server
    ADD CONSTRAINT cmdb_server_data_center_id_fkey FOREIGN KEY (data_center_id) REFERENCES public.cmdb_data_center(id) ON DELETE SET NULL;


-- Name: cmdb_server_ip cmdb_server_ip_isp_provider_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.cmdb_server_ip
    ADD CONSTRAINT cmdb_server_ip_isp_provider_id_fkey FOREIGN KEY (isp_provider_id) REFERENCES public.cmdb_provider(id) ON DELETE SET NULL;


-- Name: cmdb_server_ip cmdb_server_ip_server_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.cmdb_server_ip
    ADD CONSTRAINT cmdb_server_ip_server_id_fkey FOREIGN KEY (server_id) REFERENCES public.cmdb_server(id) ON DELETE CASCADE;


-- Name: cmdb_server cmdb_server_isp_provider_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.cmdb_server
    ADD CONSTRAINT cmdb_server_isp_provider_id_fkey FOREIGN KEY (isp_provider_id) REFERENCES public.cmdb_provider(id) ON DELETE SET NULL;


-- Name: cmdb_server cmdb_server_maintainer_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.cmdb_server
    ADD CONSTRAINT cmdb_server_maintainer_id_fkey FOREIGN KEY (maintainer_id) REFERENCES public."user"(id) ON DELETE SET NULL;


-- Name: cmdb_server_port cmdb_server_port_server_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.cmdb_server_port
    ADD CONSTRAINT cmdb_server_port_server_id_fkey FOREIGN KEY (server_id) REFERENCES public.cmdb_server(id) ON DELETE CASCADE;


-- Name: cmdb_server cmdb_server_server_provider_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.cmdb_server
    ADD CONSTRAINT cmdb_server_server_provider_id_fkey FOREIGN KEY (server_provider_id) REFERENCES public.cmdb_provider(id) ON DELETE SET NULL;


-- Name: cmdb_server cmdb_server_software_provider_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.cmdb_server
    ADD CONSTRAINT cmdb_server_software_provider_id_fkey FOREIGN KEY (software_provider_id) REFERENCES public.cmdb_provider(id) ON DELETE SET NULL;


-- Name: role_permission role_permission_role_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.role_permission
    ADD CONSTRAINT role_permission_role_id_fkey FOREIGN KEY (role_id) REFERENCES public.role(id) ON DELETE CASCADE;


-- Name: user_role user_role_role_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.user_role
    ADD CONSTRAINT user_role_role_id_fkey FOREIGN KEY (role_id) REFERENCES public.role(id) ON DELETE CASCADE;


-- Name: user_role user_role_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -

ALTER TABLE ONLY public.user_role
    ADD CONSTRAINT user_role_user_id_fkey FOREIGN KEY (user_id) REFERENCES public."user"(id) ON DELETE CASCADE;




