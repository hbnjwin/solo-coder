package com.linkyoyo.tobacco.entity.neo4j;

import org.neo4j.ogm.annotation.*;

/**
 * 规范性引用文件节点实体
 */
@NodeEntity(label = "NormativeReference")
public class NormativeReferenceNode {
    @Id
    @GeneratedValue
    private Long id;
    
    @Property("standardId")
    private String standardId; // 标准文档ID
    
    @Property("referenceStandardNo")
    private String referenceStandardNo; // 引用的标准号
    
    @Property("referenceStandardName")
    private String referenceStandardName; // 引用的标准名称
    
    @Property("referenceType")
    private String referenceType; // 引用类型
    
    @Property("referenceStatus")
    private String referenceStatus; // 引用状态
    
    @Property("referenceYear")
    private String referenceYear; // 引用年份
    
    @Relationship(type = "REFERENCED_BY", direction = Relationship.INCOMING)
    private EnhancedStandardNode referencingStandard; // 引用该标准的文档

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
    
    public String getReferenceStandardNo() {
        return referenceStandardNo;
    }
    
    public void setReferenceStandardNo(String referenceStandardNo) {
        this.referenceStandardNo = referenceStandardNo;
    }
    
    public String getReferenceStandardName() {
        return referenceStandardName;
    }
    
    public void setReferenceStandardName(String referenceStandardName) {
        this.referenceStandardName = referenceStandardName;
    }
    
    public String getReferenceType() {
        return referenceType;
    }
    
    public void setReferenceType(String referenceType) {
        this.referenceType = referenceType;
    }
    
    public String getReferenceStatus() {
        return referenceStatus;
    }
    
    public void setReferenceStatus(String referenceStatus) {
        this.referenceStatus = referenceStatus;
    }
    
    public String getReferenceYear() {
        return referenceYear;
    }
    
    public void setReferenceYear(String referenceYear) {
        this.referenceYear = referenceYear;
    }
    
    public EnhancedStandardNode getReferencingStandard() {
        return referencingStandard;
    }
    
    public void setReferencingStandard(EnhancedStandardNode referencingStandard) {
        this.referencingStandard = referencingStandard;
    }
}
