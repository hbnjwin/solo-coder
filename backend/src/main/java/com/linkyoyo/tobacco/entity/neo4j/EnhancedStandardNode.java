package com.linkyoyo.tobacco.entity.neo4j;

import org.neo4j.ogm.annotation.*;

import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;
import java.util.Set;

/**
 * 增强的标准文档节点实体
 * 包含更多标准文档属性
 */
@NodeEntity(label = "Standard")
public class EnhancedStandardNode {
    @Id
    @GeneratedValue
    private Long id;

    @Property("standardId")
    private String standardId; // 标准文档ID

    @Property("standardName")
    private String standardName; // 标准名称

    @Property("standardEnName")
    private String standardEnName; // 标准英文名

    @Property("systemName")
    private String systemName; // 标准体系名称

    @Property("standardNo")
    private String standardNo; // 标准号

    @Property("publishDate")
    private String publishDate; // 发布日期

    @Property("statusLabel")
    private String statusLabel; // 标准状态

    @Property("standardLevelName")
    private String standardLevelName; // 标准级别

    @Property("standardDeparmentName")
    private String standardDeparmentName; // 归属分标委

    @Property("deptName")
    private String deptName; // 上传组织

    @Property("publishUnit")
    private String publishUnit; // 发布单位

    @Property("implementDate")
    private String implementDate; // 实施日期

    @Property("draftUnit")
    private String draftUnit; // 起草单位

    @Property("drafter")
    private String drafter; // 起草人

    @Property("proposeUnit")
    private String proposeUnit; // 提出单位

    @Property("scope")
    private String scope; // 范围

    @Property("preface")
    private String preface; // 前言

    @Property("introduction")
    private String introduction; // 引言

    @Property("referenceDocs")
    private String referenceDocs; // 参考文献

    @Property("normativeReferences")
    private String normativeReferences; // 规范性引用文件

    @Property("termsAndDefinitions")
    private String termsAndDefinitions; // 术语和定义

    @Property("mdContent")
    private String mdContent; // Markdown文档内容

    @Property("title")
    private String title; // 标题

    @Property("type")
    private String type; // 节点类型：DOCUMENT(文档), SECTION(章节), SUBSECTION(子章节)

    @Property("content")
    private String content; // 内容

    @Property("number")
    private String number; // 章节编号，如 "4.1"

    @Property("level")
    private Integer level; // 章节级别，如 1, 2, 3

    @Property("keywords")
    private Set<String> keywords = new HashSet<>(); // 关键词

    @Relationship(type = "CONTAINS", direction = Relationship.OUTGOING)
    private List<EnhancedStandardNode> children = new ArrayList<>(); // 子节点

    @Relationship(type = "REFERENCES", direction = Relationship.OUTGOING)
    private List<EnhancedStandardNode> references = new ArrayList<>(); // 引用的节点

    @Relationship(type = "RELATES_TO", direction = Relationship.UNDIRECTED)
    private List<EnhancedStandardNode> relatedStandards = new ArrayList<>(); // 相关标准

    @Relationship(type = "HAS_TOC", direction = Relationship.OUTGOING)
    private List<TocNode> tocEntries = new ArrayList<>(); // 目录条目

    @Relationship(type = "NORMATIVELY_REFERENCES", direction = Relationship.OUTGOING)
    private List<NormativeReferenceNode> normativeReferenceNodes = new ArrayList<>(); // 规范性引用文件节点

    @Relationship(type = "BELONGS_TO", direction = Relationship.OUTGOING)
    private List<SystemNode> systems = new ArrayList<>(); // 标准体系

    @Relationship(type = "HAS_LEVEL", direction = Relationship.OUTGOING)
    private List<StandardLevelNode> levels = new ArrayList<>(); // 标准级别

    @Relationship(type = "BELONGS_TO_DEPARTMENT", direction = Relationship.OUTGOING)
    private List<StandardDepartmentNode> departments = new ArrayList<>(); // 归属分标委

    @Relationship(type = "PUBLISHED_BY", direction = Relationship.OUTGOING)
    private List<UnitNode> publishUnits = new ArrayList<>(); // 发布单位

    @Relationship(type = "PROPOSED_BY", direction = Relationship.OUTGOING)
    private List<UnitNode> proposeUnits = new ArrayList<>(); // 提出单位

    @Relationship(type = "DRAFTED_BY", direction = Relationship.OUTGOING)
    private List<UnitNode> draftUnits = new ArrayList<>(); // 起草单位

    @Relationship(type = "DRAFTED_BY_PERSON", direction = Relationship.OUTGOING)
    private List<PersonNode> drafters = new ArrayList<>(); // 起草人

    @Property("category")
    private String category; // 分类

    @Property("organization")
    private String organization; // 发布组织

    @Property("status")
    private String status; // 状态：现行有效、已废止等

    // Getters and Setters
    public Long getId() {
        return id;
    }

    public void setId(Long id) {
        this.id = id;
    }

    public String getStandardId() {
        return standardId;
    }

    public void setStandardId(String standardId) {
        this.standardId = standardId;
    }

    public String getStandardName() {
        return standardName;
    }

    public void setStandardName(String standardName) {
        this.standardName = standardName;
    }

    public String getStandardEnName() {
        return standardEnName;
    }

    public void setStandardEnName(String standardEnName) {
        this.standardEnName = standardEnName;
    }

    public String getSystemName() {
        return systemName;
    }

    public void setSystemName(String systemName) {
        this.systemName = systemName;
    }

    public String getStandardNo() {
        return standardNo;
    }

    public void setStandardNo(String standardNo) {
        this.standardNo = standardNo;
    }

    public String getPublishDate() {
        return publishDate;
    }

    public void setPublishDate(String publishDate) {
        this.publishDate = publishDate;
    }

    public String getStatusLabel() {
        return statusLabel;
    }

    public void setStatusLabel(String statusLabel) {
        this.statusLabel = statusLabel;
    }

    public String getStandardLevelName() {
        return standardLevelName;
    }

    public void setStandardLevelName(String standardLevelName) {
        this.standardLevelName = standardLevelName;
    }

    public String getStandardDeparmentName() {
        return standardDeparmentName;
    }

    public void setStandardDeparmentName(String standardDeparmentName) {
        this.standardDeparmentName = standardDeparmentName;
    }

    public String getDeptName() {
        return deptName;
    }

    public void setDeptName(String deptName) {
        this.deptName = deptName;
    }

    public String getPublishUnit() {
        return publishUnit;
    }

    public void setPublishUnit(String publishUnit) {
        this.publishUnit = publishUnit;
    }

    public String getImplementDate() {
        return implementDate;
    }

    public void setImplementDate(String implementDate) {
        this.implementDate = implementDate;
    }

    public String getDraftUnit() {
        return draftUnit;
    }

    public void setDraftUnit(String draftUnit) {
        this.draftUnit = draftUnit;
    }

    public String getDrafter() {
        return drafter;
    }

    public void setDrafter(String drafter) {
        this.drafter = drafter;
    }

    public String getProposeUnit() {
        return proposeUnit;
    }

    public void setProposeUnit(String proposeUnit) {
        this.proposeUnit = proposeUnit;
    }

    public String getScope() {
        return scope;
    }

    public void setScope(String scope) {
        this.scope = scope;
    }

    public String getPreface() {
        return preface;
    }

    public void setPreface(String preface) {
        this.preface = preface;
    }

    public String getIntroduction() {
        return introduction;
    }

    public void setIntroduction(String introduction) {
        this.introduction = introduction;
    }

    public String getReferenceDocs() {
        return referenceDocs;
    }

    public void setReferenceDocs(String referenceDocs) {
        this.referenceDocs = referenceDocs;
    }

    public String getNormativeReferences() {
        return normativeReferences;
    }

    public void setNormativeReferences(String normativeReferences) {
        this.normativeReferences = normativeReferences;
    }

    public String getTermsAndDefinitions() {
        return termsAndDefinitions;
    }

    public void setTermsAndDefinitions(String termsAndDefinitions) {
        this.termsAndDefinitions = termsAndDefinitions;
    }

    public String getMdContent() {
        return mdContent;
    }

    public void setMdContent(String mdContent) {
        this.mdContent = mdContent;
    }

    public String getTitle() {
        return title;
    }

    public void setTitle(String title) {
        this.title = title;
    }

    public String getType() {
        return type;
    }

    public void setType(String type) {
        this.type = type;
    }

    public String getContent() {
        return content;
    }

    public void setContent(String content) {
        this.content = content;
    }

    public String getNumber() {
        return number;
    }

    public void setNumber(String number) {
        this.number = number;
    }

    public Set<String> getKeywords() {
        return keywords;
    }

    public void setKeywords(Set<String> keywords) {
        this.keywords = keywords;
    }

    public List<EnhancedStandardNode> getChildren() {
        return children;
    }

    public void setChildren(List<EnhancedStandardNode> children) {
        this.children = children;
    }

    public List<EnhancedStandardNode> getReferences() {
        return references;
    }

    public void setReferences(List<EnhancedStandardNode> references) {
        this.references = references;
    }

    public List<EnhancedStandardNode> getRelatedStandards() {
        return relatedStandards;
    }

    public void setRelatedStandards(List<EnhancedStandardNode> relatedStandards) {
        this.relatedStandards = relatedStandards;
    }

    public List<TocNode> getTocEntries() {
        return tocEntries;
    }

    public void setTocEntries(List<TocNode> tocEntries) {
        this.tocEntries = tocEntries;
    }

    public List<NormativeReferenceNode> getNormativeReferenceNodes() {
        return normativeReferenceNodes;
    }

    public void setNormativeReferenceNodes(List<NormativeReferenceNode> normativeReferenceNodes) {
        this.normativeReferenceNodes = normativeReferenceNodes;
    }

    public String getCategory() {
        return category;
    }

    public void setCategory(String category) {
        this.category = category;
    }

    public Integer getLevel() {
        return level;
    }

    public void setLevel(Integer level) {
        this.level = level;
    }

    public String getOrganization() {
        return organization;
    }

    public void setOrganization(String organization) {
        this.organization = organization;
    }

    public String getStatus() {
        return status;
    }

    public void setStatus(String status) {
        this.status = status;
    }

    public List<SystemNode> getSystems() {
        return systems;
    }

    public void setSystems(List<SystemNode> systems) {
        this.systems = systems;
    }

    public List<StandardLevelNode> getLevels() {
        return levels;
    }

    public void setLevels(List<StandardLevelNode> levels) {
        this.levels = levels;
    }

    public List<StandardDepartmentNode> getDepartments() {
        return departments;
    }

    public void setDepartments(List<StandardDepartmentNode> departments) {
        this.departments = departments;
    }

    public List<UnitNode> getPublishUnits() {
        return publishUnits;
    }

    public void setPublishUnits(List<UnitNode> publishUnits) {
        this.publishUnits = publishUnits;
    }

    public List<UnitNode> getProposeUnits() {
        return proposeUnits;
    }

    public void setProposeUnits(List<UnitNode> proposeUnits) {
        this.proposeUnits = proposeUnits;
    }

    public List<UnitNode> getDraftUnits() {
        return draftUnits;
    }

    public void setDraftUnits(List<UnitNode> draftUnits) {
        this.draftUnits = draftUnits;
    }

    public List<PersonNode> getDrafters() {
        return drafters;
    }

    public void setDrafters(List<PersonNode> drafters) {
        this.drafters = drafters;
    }
}
