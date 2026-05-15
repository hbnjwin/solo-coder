package com.linkyoyo.tobacco.entity.neo4j;

import org.neo4j.ogm.annotation.*;

import java.util.ArrayList;
import java.util.List;

/**
 * 标准文档目录节点实体
 */
@NodeEntity(label = "TableOfContents")
public class TocNode {
    @Id
    @GeneratedValue
    private Long id;
    
    @Property("standardId")
    private String standardId; // 标准文档ID
    
    @Property("sectionNumber")
    private String sectionNumber; // 章节编号
    
    @Property("sectionTitle")
    private String sectionTitle; // 章节标题
    
    @Property("pageNumber")
    private String pageNumber; // 页码
    
    @Property("displayOrder")
    private Integer displayOrder; // 显示顺序
    
    @Property("level")
    private Integer level; // 章节级别
    
    @Relationship(type = "BELONGS_TO", direction = Relationship.OUTGOING)
    private EnhancedStandardNode standard; // 所属标准文档
    
    @Relationship(type = "PARENT_OF", direction = Relationship.OUTGOING)
    private List<TocNode> children = new ArrayList<>(); // 子目录

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

    public String getSectionNumber() {
        return sectionNumber;
    }

    public void setSectionNumber(String sectionNumber) {
        this.sectionNumber = sectionNumber;
    }

    public String getSectionTitle() {
        return sectionTitle;
    }

    public void setSectionTitle(String sectionTitle) {
        this.sectionTitle = sectionTitle;
    }

    public String getPageNumber() {
        return pageNumber;
    }

    public void setPageNumber(String pageNumber) {
        this.pageNumber = pageNumber;
    }

    public Integer getDisplayOrder() {
        return displayOrder;
    }

    public void setDisplayOrder(Integer displayOrder) {
        this.displayOrder = displayOrder;
    }

    public Integer getLevel() {
        return level;
    }

    public void setLevel(Integer level) {
        this.level = level;
    }

    public EnhancedStandardNode getStandard() {
        return standard;
    }

    public void setStandard(EnhancedStandardNode standard) {
        this.standard = standard;
    }

    public List<TocNode> getChildren() {
        return children;
    }

    public void setChildren(List<TocNode> children) {
        this.children = children;
    }
}
